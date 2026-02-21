use tokio_test::assert_ready;
use tokio_test::task;
use tokio_util::codec::*;

use futures::{pin_mut, Stream};

use crate::frame::*;

macro_rules! assert_next_eq {
    ($io:ident, $expect:expr) => {{
        task::spawn(()).enter(|cx, _| {
            let res = assert_ready!($io.as_mut().poll_next(cx));
            match res {
                Some(Ok(v)) => assert_eq!(v, $expect.as_ref()),
                Some(Err(e)) => panic!("error = {:?}", e),
                None => panic!("none"),
            }
        });
    }};
}

// macro_rules! assert_next_pending {
//     ($io:ident) => {{
//         task::spawn(()).enter(|cx, _| match $io.as_mut().poll_next(cx) {
//             Ready(Some(Ok(v))) => panic!("value = {:?}", v),
//             Ready(Some(Err(e))) => panic!("error = {:?}", e),
//             Ready(None) => panic!("done"),
//             Pending => {}
//         });
//     }};
// }

// macro_rules! assert_next_err {
//     ($io:ident) => {{
//         task::spawn(()).enter(|cx, _| match $io.as_mut().poll_next(cx) {
//             Ready(Some(Ok(v))) => panic!("value = {:?}", v),
//             Ready(Some(Err(_))) => {}
//             Ready(None) => panic!("done"),
//             Pending => panic!("pending"),
//         });
//     }};
// }

macro_rules! assert_done {
    ($io:ident) => {{
        task::spawn(()).enter(|cx, _| {
            let res = assert_ready!($io.as_mut().poll_next(cx));
            match res {
                Some(Ok(v)) => panic!("value = {:?}", v),
                Some(Err(e)) => panic!("error = {:?}", e),
                None => {}
            }
        });
    }};
}

// ======================
// =====    Test    =====
// ======================

use tokio_test::io::Builder;

#[test]
fn quasselcodec_set_options() {
    use flate2::Compression;

    let mut codec = QuasselCodec::default();

    assert_eq!(codec.max_frame_length(), 64 * 1024 * 1024);
    assert_eq!(codec.compression(), false);
    assert_eq!(codec.compression_level(), Compression::default());

    codec.set_max_frame_length(42);
    codec.set_compression(true);
    codec.set_compression_level(Compression::best());

    assert_eq!(codec.max_frame_length(), 42);
    assert_eq!(codec.compression(), true);
    assert_eq!(codec.compression_level(), Compression::best());
}

#[test]
fn quasselcodec_write_oversized() {
    use futures::sink::SinkExt;

    let mut io = Framed::new(
        Builder::new()
            .write_error(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                QuasselCodecError { _priv: () },
            ))
            .build(),
        QuasselCodec::builder().max_frame_length(5).new_codec(),
    );

    tokio_test::block_on(async move {
        let res = io.send(b"abcdefghi".to_vec()).await.map_err(|e| e.kind());
        let want = Err(std::io::ErrorKind::InvalidInput);

        assert_eq!(want, res);
    });
}

#[test]
fn quasselcodec_read_oversized() {
    use futures::stream::StreamExt;

    let mut io = FramedRead::new(
        Builder::new().read(b"\x00\x00\x00\x09abcdefghi").build(),
        QuasselCodec::builder().max_frame_length(5).new_codec(),
    );

    tokio_test::block_on(async move {
        let res = io.next().await.unwrap().map_err(|e| e.kind());
        let want = Err(std::io::ErrorKind::InvalidData);

        assert_eq!(want, res);
    });
}

#[test]
pub fn read_single_frame() {
    let io = FramedRead::new(
        Builder::new().read(b"\x00\x00\x00\x09abcdefghi").build(),
        QuasselCodec::new(),
    );
    pin_mut!(io);

    assert_next_eq!(io, b"abcdefghi");
    assert_done!(io);
}

#[test]
pub fn read_multi_frame() {
    let mut d: Vec<u8> = vec![];
    d.extend_from_slice(b"\x00\x00\x00\x09abcdefghi");
    d.extend_from_slice(b"\x00\x00\x00\x03123");
    d.extend_from_slice(b"\x00\x00\x00\x0bhello world");

    let io = FramedRead::new(Builder::new().read(&d).build(), QuasselCodec::new());
    pin_mut!(io);

    assert_next_eq!(io, b"abcdefghi");
    assert_next_eq!(io, b"123");
    assert_next_eq!(io, b"hello world");
    assert_done!(io);
}

#[test]
pub fn read_single_frame_compressed() {
    let io = FramedRead::new(
        Builder::new()
            .read(b"\x78\x9c\x63\x60\x60\xe0\x4c\x4c\x4a\x4e\x49\x4d\x4b\xcf\xc8\x04\x00\x11\xec\x03\x97")
            .build(),
        QuasselCodec::builder().compression(true).new_codec(),
    );
    pin_mut!(io);

    assert_next_eq!(io, b"abcdefghi");
    assert_done!(io);
}

#[test]
fn write_single_frame() {
    use futures::sink::SinkExt;

    let mut io = Framed::new(
        Builder::new().write(b"\x00\x00\x00\x09abcdefghi").build(),
        QuasselCodec::new(),
    );

    tokio_test::block_on(async move {
        io.send(b"abcdefghi".to_vec()).await.unwrap();
    });
}

#[test]
fn write_multi_frame() {
    use futures::sink::SinkExt;

    let mut io = Framed::new(
        Builder::new()
            .write(b"\x00\x00\x00\x09abcdefghi")
            .write(b"\x00\x00\x00\x03123")
            .write(b"\x00\x00\x00\x0bhello world")
            .build(),
        QuasselCodec::new(),
    );

    tokio_test::block_on(async move {
        io.send(b"abcdefghi".to_vec()).await.unwrap();
        io.send(b"123".to_vec()).await.unwrap();
        io.send(b"hello world".to_vec()).await.unwrap();
    });
}

#[test]
fn write_single_frame_compressed() {
    use futures::sink::SinkExt;

    let mut io = Framed::new(
        Builder::new()
            .write(&[120, 156, 98, 96, 96, 224, 76, 76, 74, 78, 73, 77, 75])
            .build(),
        QuasselCodec::builder().compression(true).new_codec(),
    );

    tokio_test::block_on(async move {
        io.send(b"abcdefghi".to_vec()).await.unwrap();
    });
}
