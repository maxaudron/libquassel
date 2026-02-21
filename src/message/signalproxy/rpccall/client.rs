use crate::primitive::Variant;

use super::{Direction, RpcCallType};

#[derive(Clone, Debug, PartialEq)]
pub struct KickClient {
    /// Client id of client to be kicked. Ids can be found in [CoreInfo]
    id: i32,
}

impl RpcCallType for KickClient {
    const NAME: &str = "2kickClient(int)";
    const DIRECTION: Direction = Direction::ClientToServer;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            self.id.into(),
        ])
    }

    fn from_network(
        size: usize,
        input: &mut crate::primitive::VariantList,
    ) -> Result<(usize, super::RpcCall), crate::ProtocolError>
    where
        Self: Sized,
    {
        Ok((
            size,
            Self {
                id: input.remove(0).try_into()?,
            }
            .into(),
        ))
    }
}

/// Requests the current client to disconnect from the core. Only this client sees this message.
#[derive(Clone, Debug, PartialEq)]
pub struct DisconnectFromCore;

impl RpcCallType for DisconnectFromCore {
    const NAME: &str = "2disconnectFromCore()";
    const DIRECTION: Direction = Direction::ServerToClient;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![Variant::ByteArray(Self::NAME.to_string())])
    }

    fn from_network(
        size: usize,
        _input: &mut crate::primitive::VariantList,
    ) -> Result<(usize, super::RpcCall), crate::ProtocolError>
    where
        Self: Sized,
    {
        Ok((size, Self {}.into()))
    }
}
