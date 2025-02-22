use crate::error::ProtocolError;

/// Data received right after initializing the connection
///
/// ConnAck is serialized sequentially
#[derive(Debug)]
pub struct ConnAck {
    /// The Flag 0x01 for TLS
    /// and 0x02 for Deflate Compression
    pub flags: u8,
    /// Some extra protocol version specific data
    /// So far unused
    pub extra: i16,
    /// The version of the protocol
    /// 0x00000001 for the legacy protocol
    /// 0x00000002 for the datastream protocol
    ///
    /// Only the datastream protocol is supported by this crate
    pub version: i8,
}

impl Default for ConnAck {
    fn default() -> Self {
        Self {
            flags: 0x00,
            extra: 0x00,
            version: 0x00000002,
        }
    }
}

impl crate::serialize::Serialize for ConnAck {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, ProtocolError> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.append(&mut self.flags.serialize()?);
        bytes.append(&mut self.extra.serialize()?);
        bytes.append(&mut self.version.serialize()?);

        Ok(bytes)
    }
}

impl crate::deserialize::Deserialize for ConnAck {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let (flen, flags) = u8::parse(b)?;
        let (elen, extra) = i16::parse(&b[flen..])?;
        let (vlen, version) = i8::parse(&b[(flen + elen)..])?;

        return Ok((
            flen + elen + vlen,
            Self {
                flags,
                extra,
                version,
            },
        ));
    }
}
