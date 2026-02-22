#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BufferId(pub i32);

use crate::message::NetworkList;
use crate::{error::ProtocolError, serialize::*};

use crate::serialize::UserType;

use super::Variant;

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
        Ok((size, BufferId(value)))
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

// TODO this is not correct usage, it's technically not really network repr were converting from
// but just the conversion of VariantList -> Self directly
impl NetworkList for Vec<BufferId> {
    fn to_network_list(&self) -> Result<super::VariantList, ProtocolError> {
        Ok(self.iter().map(|b| Variant::BufferId(*b)).collect())
    }

    fn from_network_list(input: super::VariantList) -> Result<Self, ProtocolError> {
        input.into_iter().map(|b| b.try_into()).collect()
    }
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
