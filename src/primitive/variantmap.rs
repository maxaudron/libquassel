use std::collections::HashMap;
use std::vec::Vec;

use log::trace;

use crate::error::ProtocolError;
use crate::serialize::*;

use crate::primitive::Variant;
use crate::util;

use crate::serialize::VariantType;

/// VariantMaps are represented as a HashMap with String as key and Variant as value
///
/// They are serialized as the amount of keys as an i32 then for each entry a String and a Variant.
pub type VariantMap = HashMap<String, Variant>;

impl Serialize for VariantMap {
    fn serialize<'a>(&'a self) -> Result<Vec<u8>, ProtocolError> {
        let mut res: Vec<u8> = Vec::new();

        for (k, v) in self {
            res.extend(k.serialize()?);
            res.extend(v.serialize()?);
        }

        let len: i32 = self.len().try_into()?;
        util::insert_bytes(0, &mut res, &mut len.to_be_bytes());

        return Ok(res);
    }
}

impl Deserialize for VariantMap {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let (_, len) = i32::parse(&b[0..4])?;
        trace!(target: "primitive::VariantMap", "Parsing VariantMap with {:?} elements", len);

        let mut pos: usize = 4;
        let mut map = VariantMap::new();
        for _ in 0..len {
            trace!(target: "primitive::VariantMap", "Parsing entry name {:x?}", &b[pos..]);
            let (nlen, name) = String::parse(&b[pos..])?;
            pos += nlen;

            trace!(target: "primitive::VariantMap", "Parsing entry: {:?} with type {:x?}", name, &b[(pos)..(pos + 4)]);
            let (vlen, value) = Variant::parse(&b[(pos)..])?;
            pos += vlen;

            map.insert(name, value);
        }

        return Ok((pos, map));
    }
}

impl VariantType for VariantMap {
    const TYPE: u32 = crate::primitive::QVARIANTMAP;
}
