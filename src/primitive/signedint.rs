use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

use std::result::Result;
use std::vec::Vec;

use crate::{error::ProtocolError, primitive, serialize::*};

use crate::serialize::VariantType;

impl Serialize for i64 {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl Deserialize for i64 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let mut rdr = Cursor::new(&b[0..8]);
        Ok((8, rdr.read_i64::<BigEndian>()?))
    }
}

impl VariantType for i64 {
    const TYPE: u32 = primitive::LONG;
}

impl Serialize for i32 {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl Deserialize for i32 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let mut rdr = Cursor::new(&b[0..4]);
        Ok((4, rdr.read_i32::<BigEndian>()?))
    }
}

impl VariantType for i32 {
    const TYPE: u32 = primitive::INT;
}

impl Serialize for i16 {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl Deserialize for i16 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let mut rdr = Cursor::new(&b[0..2]);
        Ok((2, rdr.read_i16::<BigEndian>()?))
    }
}

impl VariantType for i16 {
    const TYPE: u32 = primitive::SHORT;
}

impl Serialize for i8 {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl Deserialize for i8 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let mut rdr = Cursor::new(&b[0..1]);
        Ok((1, rdr.read_i8()?))
    }
}

impl VariantType for i8 {
    const TYPE: u32 = primitive::CHAR;
}
