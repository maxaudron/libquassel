//! Remote Procedure Calls between client and server
//!
//! RpcCalls are serialized to a VariantList first containing the i32 representation of
//! [MessageType::RpcCall], then a ByteArray of the RPC Call Name and the arguments of the call as their own
//! types.
//!
//! In this module you can find structs for each RPC Call as well as the [RpcCall] enum that implements the
//! mapping and de-/serialization. Each struct has an impl of [RpcCallType] that has the meta information for
//! the name of the rpc call as well as it's direction

use crate::error::ProtocolError;
use crate::message::MessageType;
use crate::primitive::{Variant, VariantList};
use crate::serialize::{Deserialize, Serialize};

use libquassel_derive::From;

mod bufferinfo;
mod client;
mod identity;
mod message;
mod network;
mod objectrenamed;
mod passwordchange;

pub use bufferinfo::*;
pub use client::*;
pub use identity::*;
pub use message::*;
pub use network::*;
pub use objectrenamed::*;
pub use passwordchange::*;

#[derive(Clone, Debug, PartialEq, From)]
pub enum RpcCall {
    DisplayMessage(DisplayMessage),
    DisplayStatusMessage(DisplayStatusMessage),
    SendInput(SendInput),
    CreateIdentity(CreateIdentity),
    RemoveIdentity(RemoveIdentity),
    IdentityCreated(IdentityCreated),
    IdentityRemoved(IdentityRemoved),
    CreateNetwork(CreateNetwork),
    RemoveNetwork(RemoveNetwork),
    NetworkCreated(NetworkCreated),
    NetworkRemoved(NetworkRemoved),
    ChangePassword(ChangePassword),
    PasswordChanged(PasswordChanged),
    KickClient(KickClient),
    DisconnectFromCore(DisconnectFromCore),
    ObjectRename(ObjectRenamed),
    BufferInfoUpdated(BufferInfoUpdated),
    NotImplemented,
}

/// Direction of RPC Call
pub enum Direction {
    ClientToServer,
    ServerToClient,
}

pub trait RpcCallType {
    /// String name of the call. Serialized as a [Variant::ByteArray]
    const NAME: &str;
    /// Whether the call is from the server to the client, or client to server
    const DIRECTION: Direction;

    /// Convert Self to network representation including it's [Self::NAME]
    fn to_network(&self) -> Result<Vec<Variant>, ProtocolError>;
    /// Create Self from network representation
    fn from_network(size: usize, input: &mut VariantList) -> Result<(usize, RpcCall), crate::ProtocolError>
    where
        Self: Sized;
}

impl Serialize for RpcCall {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, ProtocolError> {
        let mut res = VariantList::new();

        res.push(Variant::i32(MessageType::RpcCall as i32));

        match self {
            RpcCall::DisplayMessage(msg) => res.extend(msg.to_network()?),
            RpcCall::DisplayStatusMessage(msg) => res.extend(msg.to_network()?),
            RpcCall::SendInput(msg) => res.extend(msg.to_network()?),
            RpcCall::CreateIdentity(msg) => res.extend(msg.to_network()?),
            RpcCall::RemoveIdentity(msg) => res.extend(msg.to_network()?),
            RpcCall::IdentityCreated(msg) => res.extend(msg.to_network()?),
            RpcCall::IdentityRemoved(msg) => res.extend(msg.to_network()?),
            RpcCall::CreateNetwork(msg) => res.extend(msg.to_network()?),
            RpcCall::RemoveNetwork(msg) => res.extend(msg.to_network()?),
            RpcCall::NetworkCreated(msg) => res.extend(msg.to_network()?),
            RpcCall::NetworkRemoved(msg) => res.extend(msg.to_network()?),
            RpcCall::ChangePassword(msg) => res.extend(msg.to_network()?),
            RpcCall::PasswordChanged(msg) => res.extend(msg.to_network()?),
            RpcCall::KickClient(msg) => res.extend(msg.to_network()?),
            RpcCall::DisconnectFromCore(msg) => res.extend(msg.to_network()?),
            RpcCall::ObjectRename(msg) => res.extend(msg.to_network()?),
            RpcCall::BufferInfoUpdated(msg) => res.extend(msg.to_network()?),
            RpcCall::NotImplemented => todo!(),
        }

        res.serialize()
    }
}

impl Deserialize for RpcCall {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), ProtocolError> {
        let (size, mut res) = VariantList::parse(b)?;

        res.remove(0);

        let rpc: String = res.remove(0).try_into()?;

        match rpc.as_str() {
            DisplayMessage::NAME => DisplayMessage::from_network(size, &mut res),
            DisplayStatusMessage::NAME => DisplayStatusMessage::from_network(size, &mut res),
            SendInput::NAME => SendInput::from_network(size, &mut res),
            CreateIdentity::NAME => CreateIdentity::from_network(size, &mut res),
            RemoveIdentity::NAME => RemoveIdentity::from_network(size, &mut res),
            IdentityCreated::NAME => IdentityCreated::from_network(size, &mut res),
            IdentityRemoved::NAME => IdentityRemoved::from_network(size, &mut res),
            CreateNetwork::NAME => CreateNetwork::from_network(size, &mut res),
            RemoveNetwork::NAME => RemoveNetwork::from_network(size, &mut res),
            NetworkCreated::NAME => NetworkCreated::from_network(size, &mut res),
            NetworkRemoved::NAME => NetworkRemoved::from_network(size, &mut res),
            ChangePassword::NAME => ChangePassword::from_network(size, &mut res),
            PasswordChanged::NAME => PasswordChanged::from_network(size, &mut res),
            KickClient::NAME => KickClient::from_network(size, &mut res),
            DisconnectFromCore::NAME => DisconnectFromCore::from_network(size, &mut res),
            ObjectRenamed::NAME => ObjectRenamed::from_network(size, &mut res),
            BufferInfoUpdated::NAME => BufferInfoUpdated::from_network(size, &mut res),
            _ => Ok((size, RpcCall::NotImplemented)),
        }
    }
}
