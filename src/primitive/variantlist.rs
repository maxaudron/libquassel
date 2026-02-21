use std::vec::Vec;

use log::trace;

use crate::error::ProtocolError;
use crate::serialize::*;

use crate::primitive::Variant;

use crate::serialize::VariantType;

/// VariantLists are represented as a Vec of Variants.
///
/// They are serialized as the amount of entries as a i32 and then a Variant for each entry
pub type VariantList = Vec<Variant>;

impl Serialize for VariantList {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let len: i32 = self.len().try_into()?;
        let mut res: Vec<u8> = Vec::new();

        res.extend(len.to_be_bytes().iter());
        for v in self {
            res.extend(v.serialize()?.iter());
        }

        Ok(res)
    }
}

impl Deserialize for VariantList {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let (_, len) = i32::parse(&b[0..4])?;
        trace!(target: "primitive::VariantList", "Parsing VariantList with {:?} elements", len);

        let mut res: VariantList = VariantList::new();
        let mut pos: usize = 4;
        for i in 0..len {
            trace!(target: "primitive::VariantList", "Parsing VariantList element: {:?}", i);
            let (vlen, val) = Variant::parse(&b[pos..])?;
            trace!("parsed variant: {:?}", val);
            res.push(val);
            pos += vlen;
        }

        Ok((pos, res))
    }
}

impl VariantType for VariantList {
    const TYPE: u32 = crate::primitive::QVARIANTLIST;
}
