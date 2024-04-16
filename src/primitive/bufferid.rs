#[derive(Copy, Clone, Debug, std::cmp::PartialEq)]
pub struct BufferId(pub i32);

use failure::Error;

use crate::primitive::signedint;
use crate::{deserialize::*, serialize::*};

impl Serialize for BufferId {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        self.0.serialize()
    }
}

impl Deserialize for BufferId {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (size, value) = i32::parse(b)?;
        return Ok((size, BufferId(value)));
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
        let expected_bytes: &[u8] = &[0, 0, 0, 1];
        assert_eq!(res, expected_bytes);
    }
}
