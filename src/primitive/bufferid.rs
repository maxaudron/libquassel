#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BufferId(pub i32);

use crate::{deserialize::*, error::ProtocolError, serialize::*};

use crate::serialize::UserType;

impl Serialize for BufferId {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut res = Vec::new();

        res.append(&mut Self::NAME.serialize_utf8()?);
        res.extend(self.0.serialize()?);

        Ok(res)
    }
}

impl Deserialize for BufferId {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let (size, value) = i32::parse(b)?;
        return Ok((size, BufferId(value)));
    }
}

impl From<i32> for BufferId {
    fn from(value: i32) -> Self {
        BufferId(value)
    }
}

impl std::ops::Deref for BufferId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UserType for BufferId {
    const NAME: &str = "BufferId";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn bufferid_parse_test() {
        let test_bytes: &[u8] = &[0, 0, 0, 1];
        let (len, res) = BufferId::parse(test_bytes).unwrap();
        assert_eq!(len, test_bytes.len());
        assert_eq!(res, BufferId(1));
    }

    #[test]
    pub fn bufferid_serialize_test() {
        let res = BufferId(1).serialize().unwrap();
        let expected_bytes: &[u8] = &[0, 0, 0, 8, 66, 117, 102, 102, 101, 114, 73, 100, 0, 0, 0, 1];
        assert_eq!(res, expected_bytes);
    }
}
