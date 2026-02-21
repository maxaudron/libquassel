use crate::error::ProtocolError;
use crate::message::MessageType;
use crate::primitive::{DateTime, Variant, VariantList};
use crate::serialize::{Deserialize, Serialize};

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct HeartBeat {
    timestamp: DateTime,
}

impl Serialize for HeartBeat {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, ProtocolError> {
        vec![
            Variant::i32(MessageType::HeartBeat as i32),
            Variant::DateTime(self.timestamp),
        ]
        .serialize()
    }
}

impl Deserialize for HeartBeat {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), ProtocolError> {
        let (size, mut res) = VariantList::parse(b)?;

        res.remove(0);

        Ok((
            size,
            Self {
                timestamp: res.remove(0).try_into()?,
            },
        ))
    }
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct HeartBeatReply {
    timestamp: DateTime,
}

impl Serialize for HeartBeatReply {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, ProtocolError> {
        vec![
            Variant::i32(MessageType::HeartBeatReply as i32),
            Variant::DateTime(self.timestamp),
        ]
        .serialize()
    }
}

impl Deserialize for HeartBeatReply {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), ProtocolError> {
        let (size, mut res) = VariantList::parse(b)?;

        res.remove(0);

        Ok((
            size,
            Self {
                timestamp: res.remove(0).try_into()?,
            },
        ))
    }
}
