use std::result::Result;
use std::vec::Vec;

use crate::error::ProtocolError;
use crate::primitive::Variant;
use crate::serialize::{Deserialize, Serialize};
use crate::util;

use crate::message::handshake::{HandshakeDeserialize, HandshakeSerialize};
use crate::primitive::VariantMap;

impl HandshakeSerialize for VariantMap {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut res: Vec<u8> = Vec::new();

        for (k, v) in self {
            let key = Variant::String(k.clone());
            res.extend(key.serialize()?);
            res.extend(v.serialize()?);
        }

        let len: i32 = (self.len() * 2).try_into().unwrap();
        util::insert_bytes(0, &mut res, &mut (len).to_be_bytes());

        Ok(res)
    }
}

impl HandshakeDeserialize for VariantMap {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let (_, len) = i32::parse(&b[0..4])?;

        let mut pos: usize = 4;
        let mut map = VariantMap::new();

        for _ in 0..(len / 2) {
            let (nlen, name) = Variant::parse(&b[pos..])?;
            pos += nlen;

            let (vlen, value) = Variant::parse(&b[pos..])?;
            pos += vlen;

            match name {
                Variant::String(x) => map.insert(x, value),
                Variant::ByteArray(x) => map.insert(x, value),
                _ => return Err(ProtocolError::WrongVariant),
            };
        }

        Ok((pos, map))
    }
}

#[test]
pub fn serialize_variantmap() {
    let mut test_variantmap = VariantMap::new();
    test_variantmap.insert("Configured".to_string(), Variant::bool(true));
    let bytes = [
        0, 0, 0, 2, 0, 0, 0, 10, 0, 0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0,
        114, 0, 101, 0, 100, 0, 0, 0, 1, 0, 1,
    ]
    .to_vec();
    assert_eq!(HandshakeSerialize::serialize(&test_variantmap).unwrap(), bytes);
}

#[test]
pub fn deserialize_variantmap() {
    let test_bytes: &[u8] = &[
        0, 0, 0, 2, 0, 0, 0, 10, 0, 0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0,
        114, 0, 101, 0, 100, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1,
    ];
    let mut test_variantmap = VariantMap::new();
    test_variantmap.insert("Configured".to_string(), Variant::bool(true));

    let (len, res): (usize, VariantMap) = HandshakeDeserialize::parse(test_bytes).unwrap();

    assert_eq!(len, 39);
    assert_eq!(res, test_variantmap);
}

#[test]
pub fn deserialize_variantmap_utf8() {
    let test_bytes: &[u8] = &[
        0, 0, 0, 2, 0, 0, 0, 12, 0, 0, 0, 0, 10, 67, 111, 110, 102, 105, 103, 117, 114, 101, 100, 0, 0, 0, 1,
        0, 1, 0, 0, 0, 1,
    ];
    let mut test_variantmap = VariantMap::new();
    test_variantmap.insert("Configured".to_string(), Variant::bool(true));

    let (len, res): (usize, VariantMap) = HandshakeDeserialize::parse(test_bytes).unwrap();

    assert_eq!(len, 29);
    assert_eq!(res, test_variantmap);
}
