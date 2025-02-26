#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct MsgId(
    #[cfg(not(feature = "long-message-id"))] pub i32,
    #[cfg(feature = "long-message-id")] pub i64,
);

use crate::error::ProtocolError;
use crate::{deserialize::*, serialize::*};

impl Serialize for MsgId {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        self.0.serialize()
    }
}

impl Deserialize for MsgId {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        #[cfg(not(feature = "long-message-id"))]
        let (size, value) = i32::parse(b)?;
        #[cfg(feature = "long-message-id")]
        let (size, value) = i64::parse(b)?;
        return Ok((size, MsgId(value)));
    }
}

#[cfg(not(feature = "long-message-id"))]
impl From<i32> for MsgId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

#[cfg(feature = "long-message-id")]
impl From<i64> for MsgId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for MsgId {
    #[cfg(not(feature = "long-message-id"))]
    type Target = i32;
    #[cfg(feature = "long-message-id")]
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn msgid_parse_test() {
        let test_bytes: &[u8] = if cfg!(feature = "long-message-id") {
            &[0, 0, 0, 0, 0, 0, 0, 1]
        } else {
            &[0, 0, 0, 1]
        };
        let (len, res) = MsgId::parse(test_bytes).unwrap();
        assert_eq!(len, test_bytes.len());
        assert_eq!(res, MsgId(1));
    }

    #[test]
    pub fn msgid_serialize_test() {
        let res = MsgId(1).serialize().unwrap();
        let expected_bytes: &[u8] = if cfg!(feature = "long-message-id") {
            &[0, 0, 0, 0, 0, 0, 0, 1]
        } else {
            &[0, 0, 0, 1]
        };
        assert_eq!(res, expected_bytes);
    }
}
