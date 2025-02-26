use std::vec::Vec;

use log::trace;

use crate::error::ProtocolError;
use crate::{deserialize::*, serialize::*};

use crate::primitive::Variant;

use crate::serialize::SerializeVariant;

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

        return Ok(res);
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

        return Ok((pos, res));
    }
}

impl SerializeVariant for VariantList {
    const TYPE: u32 = crate::primitive::QVARIANTLIST;
}

impl<S> crate::message::NetworkMap for Vec<S>
where
    S: std::convert::TryFrom<Variant> + Into<Variant> + Clone + std::hash::Hash + std::cmp::Eq,
    <S as TryFrom<Variant>>::Error: std::fmt::Debug,
{
    type Item = VariantList;

    fn to_network_map(&self) -> VariantList {
        self.iter().map(|i| i.clone().into()).collect()
    }

    fn from_network_map(input: &mut VariantList) -> Self {
        input.iter().map(|i| i.clone().try_into().unwrap()).collect()
    }
}

impl<S> crate::message::NetworkList for Vec<S>
where
    S: std::convert::TryFrom<Variant> + Into<Variant> + Clone + std::hash::Hash + std::cmp::Eq,
    <S as TryFrom<Variant>>::Error: std::fmt::Debug,
{
    fn to_network_list(&self) -> VariantList {
        self.iter().map(|i| i.clone().into()).collect()
    }

    fn from_network_list(input: &mut VariantList) -> Self {
        input.iter().map(|i| i.clone().try_into().unwrap()).collect()
    }
}
