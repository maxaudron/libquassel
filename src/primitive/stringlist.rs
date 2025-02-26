extern crate byteorder;

use std::result::Result;
use std::vec::Vec;

use log::trace;

use crate::{error::ProtocolError, serialize::*};

use crate::serialize::VariantType;

/// StringList are represented as a Vec of Strings
///
/// StringLists are serialized as an i32 of the amount of elements and then each element as a String
pub type StringList = Vec<String>;

impl Serialize for StringList {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let len: i32 = self.len().try_into()?;
        let mut res: Vec<u8> = Vec::new();

        res.extend(len.to_be_bytes().iter());
        for x in self {
            res.extend(x.serialize()?);
        }

        return Ok(res);
    }
}

impl Deserialize for StringList {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let (_, len) = i32::parse(&b[0..4])?;
        trace!(target: "primitive::StringList", "Parsing with length: {:?}, from bytes: {:x?}", len, &b[0..4]);
        let mut res: StringList = StringList::new();

        let mut pos = 4;
        if len > 0 {
            for _ in 0..len {
                let (lpos, val) = String::parse(&b[pos..])?;
                pos += lpos;
                res.push(val);
            }
        }

        return Ok((pos, res));
    }
}

impl VariantType for StringList {
    const TYPE: u32 = crate::primitive::QSTRINGLIST;
}

#[test]
pub fn string_list_serialize() {
    let mut test_list = StringList::new();
    test_list.push("Configured".to_string());
    assert_eq!(
        test_list.serialize().unwrap(),
        [
            0, 0, 0, 1, 0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101,
            0, 100
        ]
    )
}

#[test]
pub fn string_list_deserialize() {
    let test_bytes: &[u8] = &[
        0, 0, 0, 1, 0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101, 0,
        100, 0, 0, 0, 1,
    ];
    let mut test_list = StringList::new();
    test_list.push("Configured".to_string());
    let (len, res) = StringList::parse(test_bytes).unwrap();
    assert_eq!(len, 28);
    assert_eq!(test_list, res);
}
