use crate::error::ProtocolError;
use crate::message::MessageType;
use crate::primitive::{Variant, VariantList};
use crate::serialize::{Deserialize, Serialize};

use super::objects::Types;

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct InitData {
    pub class_name: String,
    pub object_name: String,
    pub init_data: Types,
}

impl Serialize for InitData {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, ProtocolError> {
        let mut res = VariantList::new();

        res.push(Variant::i32(MessageType::InitData as i32));
        res.push(Variant::ByteArray(self.class_name.clone()));
        res.push(Variant::ByteArray(self.object_name.clone()));

        res.append(&mut self.init_data.to_network()?);

        res.serialize()
    }
}

impl Deserialize for InitData {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let (size, mut res) = VariantList::parse(b)?;

        res.remove(0);

        let class_name: String = res.remove(0).try_into()?;
        let object_name: String = res.remove(0).try_into()?;

        Ok((
            size,
            Self {
                class_name: class_name.clone(),
                object_name: object_name.clone(),
                init_data: Types::from_network(class_name.as_str(), object_name.as_str(), res)?,
            },
        ))
    }
}
