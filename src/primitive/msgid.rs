#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct MsgId(pub i64);

use crate::Result;
use crate::message::Feature;
use crate::serialize::*;

use crate::serialize::UserType;

impl Serialize for MsgId {
    fn serialize(&self) -> Result<Vec<u8>> {
        let mut res = Vec::new();

        res.append(&mut Self::NAME.serialize_utf8()?);
        res.extend(self.0.serialize()?);

        Ok(res)
    }
}

impl Deserialize for MsgId {
    fn parse(b: &[u8]) -> Result<(usize, Self)> {
        let (size, value) = if Feature::LongMessageId.enabled()? {
            i64::parse(b)?
        } else {
            let (size, value) = i32::parse(b)?;
            (size, value.into())
        };

        Ok((size, MsgId(value)))
    }
}

impl From<i32> for MsgId {
    fn from(value: i32) -> Self {
        Self(value.into())
    }
}

impl From<i64> for MsgId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for MsgId {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UserType for MsgId {
    const NAME: &str = "MsgId";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn msgid_parse_test() {
        let _ = Feature::enable_all();
        let test_bytes: &[u8] = if Feature::LongMessageId.enabled().unwrap() {
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
        let _ = Feature::enable_all();
        let res = MsgId(1).serialize().unwrap();
        let expected_bytes: &[u8] = if Feature::LongMessageId.enabled().unwrap() {
            &[0, 0, 0, 5, 77, 115, 103, 73, 100, 0, 0, 0, 0, 0, 0, 0, 1]
        } else {
            &[0, 0, 0, 5, 77, 115, 103, 73, 100, 0, 0, 0, 1]
        };
        assert_eq!(res, expected_bytes);
    }
}
