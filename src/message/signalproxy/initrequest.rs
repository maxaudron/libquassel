use crate::error::ProtocolError;
use crate::message::MessageType;
use crate::primitive::{Variant, VariantList};
use crate::serialize::{Deserialize, Serialize};

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct InitRequest {
    pub class_name: String,
    pub object_name: String,
}

impl Serialize for InitRequest {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, ProtocolError> {
        let mut res = VariantList::new();

        res.push(Variant::i32(MessageType::InitRequest as i32));
        res.push(Variant::ByteArray(self.class_name.clone()));
        res.push(Variant::ByteArray(self.object_name.clone()));

        res.serialize()
    }
}

impl Deserialize for InitRequest {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), ProtocolError> {
        let (size, mut res) = VariantList::parse(&b)?;

        res.remove(0);

        Ok((
            size,
            Self {
                class_name: match_variant!(res.remove(0), Variant::ByteArray),
                object_name: match_variant!(res.remove(0), Variant::ByteArray),
            },
        ))
    }
}
