extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

use std::result::Result;
use std::vec::Vec;

use crate::error::ProtocolError;
use crate::{primitive, serialize::*};

use crate::serialize::VariantType;

impl Serialize for bool {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok({
            let i = *self as i8;
            Vec::from(i.to_be_bytes())
        })
    }
}

impl Deserialize for bool {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        if b[0] == 0 {
            Ok((1, false))
        } else if b[0] == 1 {
            Ok((1, true))
        } else {
            Err(ProtocolError::BoolOutOfRange)
        }
    }
}

impl VariantType for bool {
    const TYPE: u32 = primitive::BOOL;
}

impl Serialize for u64 {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl Deserialize for u64 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let mut rdr = Cursor::new(&b[0..8]);
        return Ok((8, rdr.read_u64::<BigEndian>()?));
    }
}

impl VariantType for u64 {
    const TYPE: u32 = primitive::ULONG;
}

impl Serialize for u32 {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl Deserialize for u32 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let mut rdr = Cursor::new(&b[0..4]);
        return Ok((4, rdr.read_u32::<BigEndian>()?));
    }
}

impl VariantType for u32 {
    const TYPE: u32 = primitive::UINT;
}

impl Serialize for u16 {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl Deserialize for u16 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let mut rdr = Cursor::new(&b[0..2]);
        return Ok((2, rdr.read_u16::<BigEndian>()?));
    }
}

impl VariantType for u16 {
    const TYPE: u32 = primitive::USHORT;
}

impl Serialize for u8 {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl Deserialize for u8 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        return Ok((1, b[0]));
    }
}

impl VariantType for u8 {
    const TYPE: u32 = primitive::UCHAR;
}
