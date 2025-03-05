use crate::error::ProtocolError;
use crate::message::MessageType;
use crate::primitive::{Variant, VariantList};
use crate::serialize::{Deserialize, Serialize};

use super::Class;

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct InitRequest {
    pub class_name: Class,
    pub object_name: String,
}

impl Serialize for InitRequest {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, ProtocolError> {
        vec![
            Variant::i32(MessageType::InitRequest as i32),
            Variant::ByteArray(self.class_name.as_str().to_owned()),
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
                class_name: Class::from(TryInto::<String>::try_into(res.remove(0))?),
                object_name: res.remove(0).try_into()?,
            },
        ))
    }
}
