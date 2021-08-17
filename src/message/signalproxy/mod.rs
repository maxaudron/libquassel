use std::convert::TryInto;

use crate::{
    deserialize::Deserialize,
    primitive::{VariantList, VariantMap},
    serialize::Serialize,
    session::Session,
};

use num_derive::{FromPrimitive, ToPrimitive};

mod heartbeat;
mod initdata;
mod initrequest;
pub mod objects;
mod rpccall;
mod syncmessage;

mod translation;
pub use translation::*;

pub use heartbeat::*;
pub use initdata::*;
pub use initrequest::*;
pub use rpccall::*;
pub use syncmessage::*;

/// SyncProxy sends sync and rpc messages
pub trait SyncProxy: Session {
    fn sync(
        &self,
        class_name: &str,
        object_name: Option<&str>,
        function: &str,
        params: VariantList,
    );

    /// Send a RpcCall
    fn rpc(&self, function: &str, params: VariantList);
}

/// A base Syncable Object
pub trait Syncable {
    /// Send a SyncMessage.
    ///
    /// Must implement a call to session.sync() that sets the class and object names
    ///
    /// Example:
    /// ```ignore
    /// impl Syncable for AliasManager {
    /// fn sync(&self, session: impl SyncProxy, function: &str, params: VariantList) {
    /// session.sync("AliasManager", None, function, params)
    /// }
    /// }
    /// ```
    fn sync(&self, session: impl SyncProxy, function: &str, params: VariantList);

    /// Send a RpcCall
    fn rpc(&self, session: impl SyncProxy, function: &str, params: VariantList) {
        session.rpc(function, params);
    }
}

/// A Stateful Syncable Object
#[allow(unused_variables)]
pub trait StatefulSyncable: Syncable {
    /// Client -> Server: Update the whole object with received data
    fn update(&mut self, session: impl SyncProxy, params: VariantMap)
    where
        Self: Sized + From<VariantMap>,
    {
        #[cfg(feature = "client")]
        {
            self.sync(session, "update", vec![params.into()]);
        }
        #[cfg(feature = "server")]
        {
            *self = params.try_into().unwrap();
        }
    }

    /// Server -> Client: Update the whole object with received data
    fn request_update(&mut self, session: impl SyncProxy, params: VariantMap)
    where
        Self: Sized + From<VariantMap>,
    {
        #[cfg(feature = "client")]
        {
            *self = params.try_into().unwrap();
        }
        #[cfg(feature = "server")]
        {
            self.sync(session, "requestUpdate", vec![params.into()]);
        }
    }
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub enum Message {
    /// Bidirectional
    SyncMessage(SyncMessage),
    /// Bidirectional
    RpcCall(RpcCall),
    InitRequest(InitRequest),
    InitData(InitData),
    /// Bidirectional
    HeartBeat(HeartBeat),
    /// Bidirectional
    HeartBeatReply(HeartBeatReply),
}

// impl Message {
//     fn act(&self) {
//         match &self {
//             Message::SyncMessage(value) => value.serialize(),
//             Message::RpcCall(value) => value.serialize(),
//             Message::InitRequest(value) => value.serialize(),
//             Message::InitData(value) => value.serialize(),
//             Message::HeartBeat(value) => value.serialize(),
//             Message::HeartBeatReply(value) => value.serialize(),
//         }
//     }
// }

impl Serialize for Message {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, failure::Error> {
        match &self {
            Message::SyncMessage(value) => value.serialize(),
            Message::RpcCall(value) => value.serialize(),
            Message::InitRequest(value) => value.serialize(),
            Message::InitData(value) => value.serialize(),
            Message::HeartBeat(value) => value.serialize(),
            Message::HeartBeatReply(value) => value.serialize(),
        }
    }
}

impl Deserialize for Message {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), failure::Error> {
        let (_, message_type) = i32::parse(&b[9..13])?;

        match MessageType::from(message_type) {
            MessageType::SyncMessage => {
                let (size, res) = SyncMessage::parse(&b)?;

                Ok((size, Message::SyncMessage(res)))
            }
            MessageType::RpcCall => {
                let (size, res) = RpcCall::parse(&b)?;

                Ok((size, Message::RpcCall(res)))
            }
            MessageType::InitRequest => {
                let (size, res) = InitRequest::parse(&b)?;

                Ok((size, Message::InitRequest(res)))
            }
            MessageType::InitData => {
                let (size, res) = InitData::parse(&b)?;

                Ok((size, Message::InitData(res)))
            }
            MessageType::HeartBeat => {
                let (size, res) = HeartBeat::parse(&b)?;

                Ok((size, Message::HeartBeat(res)))
            }
            MessageType::HeartBeatReply => {
                let (size, res) = HeartBeatReply::parse(&b)?;

                Ok((size, Message::HeartBeatReply(res)))
            }
        }
    }
}

/// Type of an SignalProxy Message
/// The first element in the VariantList that is received
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum MessageType {
    /// Bidirectional
    SyncMessage = 0x00000001,
    /// Bidirectional
    RpcCall = 0x00000002,
    InitRequest = 0x00000003,
    InitData = 0x00000004,
    /// Bidirectional
    HeartBeat = 0x00000005,
    /// Bidirectional
    HeartBeatReply = 0x00000006,
}

impl From<i32> for MessageType {
    fn from(val: i32) -> Self {
        match val {
            0x00000001 => MessageType::SyncMessage,
            0x00000002 => MessageType::RpcCall,
            0x00000003 => MessageType::InitRequest,
            0x00000004 => MessageType::InitData,
            0x00000005 => MessageType::HeartBeat,
            0x00000006 => MessageType::HeartBeatReply,
            _ => unimplemented!(),
        }
    }
}
