use crate::primitive::{BufferInfo, Message, Variant};

use super::{Direction, RpcCall, RpcCallType};

/// Called when a new IRC message has been received, and the client should display or store it.
#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct DisplayMessage {
    pub message: Message,
}

impl RpcCallType for DisplayMessage {
    const NAME: &str = "2displayMessage(Message)";
    const DIRECTION: Direction = Direction::ServerToClient;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            self.message.clone().into(),
        ])
    }

    fn from_network(
        size: usize,
        input: &mut crate::primitive::VariantList,
    ) -> Result<(usize, RpcCall), crate::ProtocolError>
    where
        Self: Sized,
    {
        Ok((
            size,
            RpcCall::DisplayMessage(DisplayMessage {
                message: input.remove(0).try_into().unwrap(),
            }),
        ))
    }
}

/// Status message for an IRC network to be shown in the client’s status bar (if available).
#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct DisplayStatusMessage {
    pub network: String,
    pub message: String,
}

impl RpcCallType for DisplayStatusMessage {
    const NAME: &str = "2displayStatusMsg(QString,QString)";
    const DIRECTION: Direction = Direction::ServerToClient;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            self.network.clone().into(),
            self.message.clone().into(),
        ])
    }

    fn from_network(
        size: usize,
        input: &mut crate::primitive::VariantList,
    ) -> Result<(usize, RpcCall), crate::ProtocolError>
    where
        Self: Sized,
    {
        Ok((
            size,
            DisplayStatusMessage {
                network: input.remove(0).try_into().unwrap(),
                message: input.remove(0).try_into().unwrap(),
            }
            .into(),
        ))
    }
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct SendInput {
    buffer: BufferInfo,
    message: String,
}

impl RpcCallType for SendInput {
    const NAME: &str = "2sendInput(BufferInfo,QString)";
    const DIRECTION: Direction = Direction::ClientToServer;

    fn to_network(&self) -> Result<Vec<Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            self.buffer.clone().into(),
            self.message.clone().into(),
        ])
    }

    fn from_network(
        size: usize,
        input: &mut crate::primitive::VariantList,
    ) -> Result<(usize, RpcCall), crate::ProtocolError>
    where
        Self: Sized,
    {
        Ok((
            size,
            Self {
                buffer: input.remove(0).try_into().unwrap(),
                message: input.remove(0).try_into().unwrap(),
            }
            .into(),
        ))
    }
}
