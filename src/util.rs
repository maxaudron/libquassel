/// Prepend the length of `buf` to `buf`
pub fn prepend_byte_len(buf: &mut Vec<u8>) -> crate::Result<()> {
    let len: i32 = buf.len().try_into()?;
    let ulen: &[u8] = &len.to_be_bytes();
    buf.insert(0, ulen[3]);
    buf.insert(0, ulen[2]);
    buf.insert(0, ulen[1]);
    buf.insert(0, ulen[0]);

    Ok(())
}

/// Insert a bytes `input` into `buf` at position `pos`
pub fn insert_bytes(pos: usize, buf: &mut Vec<u8>, input: &mut [u8]) {
    input.reverse();
    for i in input {
        buf.insert(pos, *i)
    }
}

/// Easily create HashMaps for tests
///
/// ```rust
/// use libquassel::{map, s};
/// use libquassel::primitive::{Variant, VariantMap};
///
/// let example: VariantMap = map! {
///    s!("id") => Variant::VariantList(vec![Variant::i32(1)]),
///    s!("name") => Variant::StringList(vec![s!("testrule")]),
///};
/// ```
#[macro_export]
macro_rules! map {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::iter::IntoIterator::into_iter([$(($k, $v),)*]))
    };
    // set-like
    ($($v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::iter::IntoIterator::into_iter([$($v,)*]))
    };
}

/// Shorthand to make an owned string: `s!("example text")`
#[macro_export]
macro_rules! s {
    ($values:expr) => {
        std::string::String::from($values)
    };
}

/// Remove the first entry in a SyncMessage
#[macro_export]
macro_rules! get_param {
    ( $msg:expr ) => {
        $msg.params.remove(0).try_into()?
    };
}
