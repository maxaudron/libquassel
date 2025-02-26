use crate::error::ProtocolError;
use crate::message::MessageType;
use crate::primitive::Message;
use crate::primitive::{Variant, VariantList};
use crate::serialize::{Deserialize, Serialize};

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub enum RpcCall {
    DisplayMessage(DisplayMessage),
    NotImplemented,
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct DisplayMessage {
    pub message: Message,
}

// #[derive(Clone, Debug, std::cmp::PartialEq)]
// pub struct RpcCall {
//     pub slot_name: String,
//     pub params: VariantList,
// }

impl Serialize for RpcCall {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, ProtocolError> {
        let mut res = VariantList::new();

        res.push(Variant::i32(MessageType::RpcCall as i32));

        match self {
            RpcCall::DisplayMessage(msg) => {
                res.push(Variant::ByteArray("2displayMsg(Message)".to_string()));
                res.push(Variant::Message(msg.message.clone()));
            }
            RpcCall::NotImplemented => todo!(),
        }

        res.serialize()
    }
}

impl Deserialize for RpcCall {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), ProtocolError> {
        let (size, mut res) = VariantList::parse(&b)?;

        res.remove(0);

        let rpc = match_variant!(res.remove(0), Variant::ByteArray);

        match rpc.as_str() {
            "2displayMsg(Message)" => {
                return Ok((
                    size,
                    RpcCall::DisplayMessage(DisplayMessage {
                        message: match_variant!(res.remove(0), Variant::Message),
                    }),
                ))
            }
            _ => return Ok((size, RpcCall::NotImplemented)),
        }
    }
}
