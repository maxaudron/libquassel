use crate::{
    error::ProtocolError,
    primitive::{Variant, VariantList},
    serialize::Deserialize,
    serialize::Serialize,
};

use rpccall::RpcCall;

use log::debug;
use num_derive::{FromPrimitive, ToPrimitive};

mod heartbeat;
mod initdata;
mod initrequest;
pub mod objects;
pub mod rpccall;
mod syncmessage;

pub mod translation;
pub use translation::*;

pub use heartbeat::*;
pub use initdata::*;
pub use initrequest::*;
pub use syncmessage::*;

use once_cell::sync::OnceCell;

pub static SYNC_PROXY: OnceCell<SyncProxy> = OnceCell::new();

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SyncProxy {
    sync_channel: crossbeam_channel::Sender<SyncMessage>,
    rpc_channel: crossbeam_channel::Sender<RpcCall>,
}

/// SyncProxy sends sync and rpc messages
impl SyncProxy {
    /// Initialize the global SYNC_PROXY object and return receiver ends for the SyncMessage and RpcCall channels
    pub fn init(
        cap: usize,
    ) -> (
        crossbeam_channel::Receiver<SyncMessage>,
        crossbeam_channel::Receiver<RpcCall>,
    ) {
        let (sync_tx, sync_rx) = crossbeam_channel::bounded(cap);
        let (rpc_tx, rpc_rx) = crossbeam_channel::bounded(cap);

        SYNC_PROXY
            .set(SyncProxy {
                sync_channel: sync_tx,
                rpc_channel: rpc_tx,
            })
            .unwrap();

        (sync_rx, rpc_rx)
    }

    /// Send a SyncMessage
    fn sync(&self, class_name: Class, object_name: Option<&str>, function: &str, params: VariantList) {
        let msg = SyncMessage {
            class_name,
            object_name: object_name.unwrap_or("").to_string(),
            slot_name: function.to_string(),
            params,
        };

        debug!("submitting {:#?}", msg);
        self.sync_channel.send(msg).unwrap();
    }

    /// Send an RpcCall
    fn rpc(&self, _function: &str, _params: VariantList) {}
}

/// A base Syncable Object
///
/// Provides default implementations for sending SyncMessages and
/// RpcCalls so you usually only have to set the CLASS const.
///
/// If the object name has to be set implement the send_sync() function.
pub trait Syncable {
    /// The Class of the object as transmitted in the SyncMessage
    const CLASS: Class;

    /// Send a SyncMessage.
    fn send_sync(&self, function: &str, params: VariantList) {
        crate::message::signalproxy::SYNC_PROXY
            .get()
            .unwrap()
            .sync(Self::CLASS, None, function, params);
    }

    /// Send a RpcCall
    fn send_rpc(&self, function: &str, params: VariantList) {
        crate::message::signalproxy::SYNC_PROXY
            .get()
            .unwrap()
            .rpc(function, params);
    }

    fn init(&mut self, data: Self)
    where
        Self: Sized,
    {
        *self = data
    }
}

/// Methods for a Stateful Syncable object on the client side.
pub trait StatefulSyncableServer: Syncable + translation::NetworkMap
where
    Variant: From<<Self as translation::NetworkMap>::Item>,
{
    fn sync(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "requestUpdate" => {
                StatefulSyncableServer::request_update(self, msg.params.pop().unwrap().try_into().unwrap())
            }
            _ => StatefulSyncableServer::sync_custom(self, msg),
        }
    }

    #[allow(unused_mut)]
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        #[allow(clippy::match_single_binding)]
        match msg.slot_name.as_str() {
            _ => (),
        }
    }

    /// Client -> Server: Update the whole object with received data
    fn update(&mut self)
    where
        Self: Sized,
    {
        self.send_sync("update", vec![self.to_network_map().into()]);
    }

    /// Server -> Client: Update the whole object with received data
    fn request_update(&mut self, mut param: <Self as translation::NetworkMap>::Item)
    where
        Self: Sized,
    {
        *self = Self::from_network_map(&mut param);
    }
}

/// Methods for a Stateful Syncable object on the server side.
pub trait StatefulSyncableClient: Syncable + translation::NetworkMap {
    fn sync(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "update" => StatefulSyncableClient::update(self, msg.params.pop().unwrap().try_into().unwrap()),
            _ => StatefulSyncableClient::sync_custom(self, msg),
        }
    }

    #[allow(unused_mut)]
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        #[allow(clippy::match_single_binding)]
        match msg.slot_name.as_str() {
            _ => (),
        }
    }

    /// Client -> Server: Update the whole object with received data
    fn update(&mut self, mut param: <Self as translation::NetworkMap>::Item)
    where
        Self: Sized,
    {
        *self = Self::from_network_map(&mut param);
    }

    /// Server -> Client: Update the whole object with received data
    fn request_update(&mut self)
    where
        Self: Sized,
    {
        self.send_sync("requestUpdate", vec![self.to_network_map().into()]);
    }
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub enum Message {
    /// Bidirectional
    SyncMessage(SyncMessage),
    /// Bidirectional
    RpcCall(RpcCall),
    InitRequest(InitRequest),
    InitData(Box<InitData>),
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
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, ProtocolError> {
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
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), ProtocolError> {
        let (_, message_type) = i32::parse(&b[9..13])?;

        match MessageType::from(message_type) {
            MessageType::SyncMessage => {
                let (size, res) = SyncMessage::parse(b)?;

                Ok((size, Message::SyncMessage(res)))
            }
            MessageType::RpcCall => {
                let (size, res) = RpcCall::parse(b)?;

                Ok((size, Message::RpcCall(res)))
            }
            MessageType::InitRequest => {
                let (size, res) = InitRequest::parse(b)?;

                Ok((size, Message::InitRequest(res)))
            }
            MessageType::InitData => {
                let (size, res) = InitData::parse(b)?;

                Ok((size, Message::InitData(Box::new(res))))
            }
            MessageType::HeartBeat => {
                let (size, res) = HeartBeat::parse(b)?;

                Ok((size, Message::HeartBeat(res)))
            }
            MessageType::HeartBeatReply => {
                let (size, res) = HeartBeatReply::parse(b)?;

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
