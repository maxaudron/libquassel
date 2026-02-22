use std::error::Error as StdError;
use std::fmt;
use std::io::{self, Cursor};

use bytes::{Buf, BufMut, BytesMut};

use tokio::io::{AsyncRead, AsyncWrite};

use tokio_util::codec::{Decoder, Encoder, Framed, FramedRead, FramedWrite};

use flate2::Compress;
use flate2::Compression;
use flate2::Decompress;
use flate2::FlushCompress;
use flate2::FlushDecompress;

#[cfg(test)]
mod tests;

/// Builder for the QuasselCodec
#[derive(Debug, Clone, Copy)]
pub struct Builder {
    /// Enable or Disable Compression
    compression: bool,
    /// The level of Compression
    compression_level: Compression,

    /// Maximum length of the frame
    max_frame_len: usize,
}

// An error when the number of bytes read is more than max frame length.
#[derive(PartialEq)]
pub struct QuasselCodecError {
    _priv: (),
}

/// QuasselCodec provides the base layer of frameing and compression
#[derive(Debug)]
pub struct QuasselCodec {
    builder: Builder,
    state: DecodeState,
    comp: Compress,
    decomp: Decompress,
}

#[derive(Debug, Clone, Copy)]
enum DecodeState {
    Head,
    Data(usize),
}

impl QuasselCodec {
    /// Creates a new quassel codec with default values
    pub fn new() -> Self {
        Self {
            builder: Builder::new(),
            state: DecodeState::Head,
            comp: Compress::new(Compression::default(), true),
            decomp: Decompress::new(true),
        }
    }

    /// Creates a new quassel codec builder with default configuration
    /// values.
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Gets the maximum frame length
    pub fn max_frame_length(&self) -> usize {
        self.builder.max_frame_len
    }

    pub fn compression(&self) -> bool {
        self.builder.compression
    }

    pub fn compression_level(&self) -> Compression {
        self.builder.compression_level
    }

    /// Gets the maximum frame length
    pub fn set_max_frame_length(&mut self, val: usize) {
        self.builder.max_frame_length(val);
    }

    pub fn set_compression(&mut self, val: bool) {
        self.builder.compression(val);
    }

    pub fn set_compression_level(&mut self, val: Compression) {
        self.builder.compression_level(val);
    }

    fn decode_head(&mut self, src: &mut BytesMut) -> io::Result<Option<usize>> {
        let head_len = 4;

        if src.len() < head_len {
            // Not enough data
            return Ok(None);
        }

        let field_len = {
            let mut src = Cursor::new(&mut *src);

            let field_len = src.get_uint(head_len);

            if field_len > self.builder.max_frame_len as u64 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    QuasselCodecError { _priv: () },
                ));
            }

            // The check above ensures there is no overflow
            field_len as usize
        };

        // Strip header
        let _ = src.split_to(head_len);

        // Ensure that the buffer has enough space to read the incoming
        // payload
        src.reserve(field_len);

        Ok(Some(field_len))
    }

    fn decode_data(&self, n: usize, src: &mut BytesMut) -> io::Result<Option<BytesMut>> {
        // At this point, the buffer has already had the required capacity
        // reserved. All there is to do is read.
        if src.len() < n {
            return Ok(None);
        }

        Ok(Some(src.split_to(n)))
    }
}

impl Decoder for QuasselCodec {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<BytesMut>, io::Error> {
        // Create Unified Buffer for compressed and not compressed datastream
        let mut buf: &mut BytesMut = &mut BytesMut::new();

        if self.builder.compression == true {
            // Buffer to shove uncompressed stream into
            let mut msg = Vec::with_capacity(self.builder.max_frame_len);

            let before_in = self.decomp.total_in();
            let before_out = self.decomp.total_out();

            self.decomp
                .decompress_vec(&src, &mut msg, FlushDecompress::None)?;
            // Clear the src buffer, decompress() only peeks at content.
            // without this we will endlessly loop over the same frame.
            src.clear();

            let after_in = self.decomp.total_in();
            let after_out = self.decomp.total_out();

            let len = (after_out - before_out).try_into()?;

            // Reserve length of uncompressed stream
            // and put bytes into there
            buf.reserve(len);
            buf.put(&msg[..]);
        } else {
            buf = src;
        }

        let n = match self.state {
            DecodeState::Head => match self.decode_head(buf)? {
                Some(n) => {
                    self.state = DecodeState::Data(n);
                    n
                }
                None => return Ok(None),
            },
            DecodeState::Data(n) => n,
        };

        match self.decode_data(n, buf)? {
            Some(data) => {
                // Update the decode state
                self.state = DecodeState::Head;

                // Make sure the buffer has enough space to read the next head
                buf.reserve(4);

                Ok(Some(data))
            }
            None => Ok(None),
        }
    }
}

impl Encoder<Vec<u8>> for QuasselCodec {
    type Error = io::Error;

    fn encode(&mut self, data: Vec<u8>, dst: &mut BytesMut) -> Result<(), io::Error> {
        let buf = &mut BytesMut::new();

        let n = (&data).len();

        if n > self.builder.max_frame_len {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                QuasselCodecError { _priv: () },
            ));
        }

        // Reserve capacity in the destination buffer to fit the frame and
        // length field (plus adjustment).
        buf.reserve(4 + n);

        buf.put_uint(n as u64, 4);

        // Write the frame to the buffer
        buf.extend_from_slice(&data[..]);

        if self.builder.compression {
            let mut cbuf: Vec<u8> = vec![0; 4 + n];

            let before_in = self.comp.total_in();
            let before_out = self.comp.total_out();

            self.comp.compress(buf, &mut cbuf, FlushCompress::Full)?;

            let after_in = self.comp.total_in();
            let after_out = self.comp.total_out();

            cbuf.truncate((after_out - before_out).try_into()?);
            *dst = BytesMut::from(&cbuf[..]);
        } else {
            *dst = buf.clone();
        }

        Ok(())
    }
}

impl Default for QuasselCodec {
    fn default() -> Self {
        Self::new()
    }
}

// ===== impl Builder =====

impl Builder {
    /// Creates a new codec builder with default configuration
    /// values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::io::AsyncRead;
    /// use libquassel::frame::QuasselCodec;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// QuasselCodec::builder()
    ///     .new_read(io);
    /// # }
    /// # pub fn main() {}
    /// ```
    pub fn new() -> Builder {
        Builder {
            compression: false,
            compression_level: Compression::default(),
            max_frame_len: 64 * 1024 * 1024,
        }
    }

    /// Enables or disables the compression
    pub fn compression(&mut self, val: bool) -> &mut Self {
        self.compression = val;
        self
    }

    /// Sets the level of compression to
    pub fn compression_level(&mut self, val: Compression) -> &mut Self {
        self.compression_level = val;
        self
    }

    /// Sets the max frame length
    ///
    /// This configuration option applies to both encoding and decoding. The
    /// default value is 67MB.
    ///
    /// When decoding, the length field read from the byte stream is checked
    /// against this setting **before** any adjustments are applied. When
    /// encoding, the length of the submitted payload is checked against this
    /// setting.
    ///
    /// When frames exceed the max length, an `io::Error` with the custom value
    /// of the `QuasselCodecError` type will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::io::AsyncRead;
    /// use libquassel::frame::QuasselCodec;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// QuasselCodec::builder()
    ///     .max_frame_length(8 * 1024)
    ///     .new_read(io);
    /// # }
    /// # pub fn main() {}
    /// ```
    pub fn max_frame_length(&mut self, val: usize) -> &mut Self {
        self.max_frame_len = val;
        self
    }

    /// Create a configured `QuasselCodec`
    ///
    /// # Examples
    ///
    /// ```
    /// use libquassel::frame::QuasselCodec;
    /// # pub fn main() {
    /// QuasselCodec::builder()
    ///     .new_codec();
    /// # }
    /// ```
    pub fn new_codec(&self) -> QuasselCodec {
        QuasselCodec {
            builder: *self,
            state: DecodeState::Head,
            comp: Compress::new(self.compression_level, true),
            decomp: Decompress::new(true),
        }
    }

    /// Create a configured `FramedRead`
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::io::AsyncRead;
    /// use libquassel::frame::QuasselCodec;
    ///
    /// # fn bind_read<T: AsyncRead>(io: T) {
    /// QuasselCodec::builder()
    ///     .new_read(io);
    /// # }
    /// # pub fn main() {}
    /// ```
    pub fn new_read<T>(&self, upstream: T) -> FramedRead<T, QuasselCodec>
    where
        T: AsyncRead,
    {
        FramedRead::new(upstream, self.new_codec())
    }

    /// Create a configured `FramedWrite`
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::io::AsyncWrite;
    /// # use libquassel::frame::QuasselCodec;
    /// # fn write_frame<T: AsyncWrite>(io: T) {
    /// QuasselCodec::builder()
    ///     .new_write(io);
    /// # }
    /// # pub fn main() {}
    /// ```
    pub fn new_write<T>(&self, inner: T) -> FramedWrite<T, QuasselCodec>
    where
        T: AsyncWrite,
    {
        FramedWrite::new(inner, self.new_codec())
    }

    /// Create a configured `Framed`
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::io::{AsyncRead, AsyncWrite};
    /// # use libquassel::frame::QuasselCodec;
    /// # fn write_frame<T: AsyncRead + AsyncWrite>(io: T) {
    /// # let _ =
    /// QuasselCodec::builder()
    ///     .new_framed(io);
    /// # }
    /// # pub fn main() {}
    /// ```
    pub fn new_framed<T>(&self, inner: T) -> Framed<T, QuasselCodec>
    where
        T: AsyncRead + AsyncWrite,
    {
        Framed::new(inner, self.new_codec())
    }
}

// ===== impl LengthDelimitedCodecError =====

impl fmt::Debug for QuasselCodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QuasselCodecError").finish()
    }
}

impl fmt::Display for QuasselCodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("frame size too big")
    }
}

impl StdError for QuasselCodecError {}
