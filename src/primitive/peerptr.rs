use crate::{
    serialize::Deserialize,
    serialize::{Serialize, SerializeUTF8},
};

use crate::serialize::UserType;

#[derive(Copy, Clone, Debug, std::cmp::PartialEq)]
#[repr(transparent)]
pub struct PeerPtr(pub i64);

impl Serialize for PeerPtr {
    fn serialize(&self) -> Result<Vec<u8>, crate::error::ProtocolError> {
        let mut res = Vec::new();

        res.append(&mut Self::NAME.serialize_utf8()?);
        res.extend(self.0.serialize()?);

        Ok(res)
    }
}

impl Deserialize for PeerPtr {
    fn parse(b: &[u8]) -> Result<(usize, Self), crate::error::ProtocolError> {
        let (size, value) = i64::parse(b)?;
        Ok((size, PeerPtr(value)))
    }
}

impl UserType for PeerPtr {
    const NAME: &str = "PeerPtr";
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // pub fn peerptr_parse_test() {
    //     let test_bytes: &[u8] = &[
    //         0, 0, 0, 7, 80, 101, 101, 114, 80, 116, 114, 0, 0, 0, 0, 0, 0, 0, 1,
    //     ];
    //     let (len, res) = PeerPtr::parse(test_bytes).unwrap();
    //     assert_eq!(len, test_bytes.len());
    //     assert_eq!(res, PeerPtr(1));
    // }

    #[test]
    pub fn peerptr_serialize_test() {
        let res = PeerPtr(1).serialize().unwrap();
        let expected_bytes: &[u8] = &[
            0, 0, 0, 7, 80, 101, 101, 114, 80, 116, 114, 0, 0, 0, 0, 0, 0, 0, 1,
        ];
        assert_eq!(res, expected_bytes);
    }
}
