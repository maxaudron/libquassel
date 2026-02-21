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
        vec![
            Variant::i32(MessageType::InitRequest as i32),
            Variant::ByteArray(self.class_name.clone()),
            Variant::ByteArray(self.object_name.clone()),
        ]
        .serialize()
    }
}

impl Deserialize for InitRequest {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), ProtocolError> {
        let (size, mut res) = VariantList::parse(b)?;

        res.remove(0);

        Ok((
            size,
            Self {
                class_name: res.remove(0).try_into()?,
                object_name: res.remove(0).try_into()?,
            },
        ))
    }
}
