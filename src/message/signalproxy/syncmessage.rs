use crate::error::ProtocolError;
use crate::message::MessageType;
use crate::primitive::{Variant, VariantList};
use crate::serialize::{Deserialize, Serialize};

use super::Class;

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct SyncMessage {
    pub class_name: Class,
    pub object_name: String,
    pub slot_name: String,
    pub params: VariantList,
}

// impl Act for SyncMessage {}

impl Serialize for SyncMessage {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, ProtocolError> {
        let mut res = vec![
            Variant::i32(MessageType::SyncMessage as i32),
            Variant::ByteArray(self.class_name.as_str().to_owned()),
            Variant::ByteArray(self.object_name.clone()),
            Variant::ByteArray(self.slot_name.clone()),
        ];

        res.append(&mut self.params.clone());

        res.serialize()
    }
}

impl Deserialize for SyncMessage {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), ProtocolError> {
        let (size, mut res) = VariantList::parse(b)?;

        res.remove(0);

        let class_name: String = res.remove(0).try_into()?;

        Ok((
            size,
            Self {
                class_name: Class::from(class_name),
                object_name: res.remove(0).try_into()?,
                slot_name: res.remove(0).try_into()?,
                params: res,
            },
        ))
    }
}
