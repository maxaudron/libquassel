use crate::{
    message::objects::NetworkInfo,
    primitive::{NetworkId, StringList, Variant},
};

use super::{Direction, RpcCallType};

#[derive(Clone, Debug, PartialEq)]
pub struct CreateNetwork {
    network: NetworkInfo,
    channels: StringList,
}

impl RpcCallType for CreateNetwork {
    const NAME: &str = "2createNetwork(NetworkInfo,QStringList)";
    const DIRECTION: Direction = Direction::ClientToServer;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            self.network.clone().into(),
            self.channels.clone().into(),
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
                network: match_variant!(input.remove(0), Variant::NetworkInfo),
                channels: match_variant!(input.remove(0), Variant::StringList),
            }
            .into(),
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RemoveNetwork {
    network_id: NetworkId,
}

impl RpcCallType for RemoveNetwork {
    const NAME: &str = "2removeNetwork(NetworkId)";
    const DIRECTION: Direction = Direction::ClientToServer;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            self.network_id.clone().into(),
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
                network_id: match_variant!(input.remove(0), Variant::NetworkId),
            }
            .into(),
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NetworkCreated {
    network_id: NetworkId,
}

impl RpcCallType for NetworkCreated {
    const NAME: &str = "2networkCreated(NetworkId)";
    const DIRECTION: Direction = Direction::ClientToServer;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            self.network_id.clone().into(),
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
                network_id: match_variant!(input.remove(0), Variant::NetworkId),
            }
            .into(),
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NetworkRemoved {
    network_id: NetworkId,
}

impl RpcCallType for NetworkRemoved {
    const NAME: &str = "2networkRemoved(NetworkId)";
    const DIRECTION: Direction = Direction::ClientToServer;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            self.network_id.clone().into(),
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
                network_id: match_variant!(input.remove(0), Variant::NetworkId),
            }
            .into(),
        ))
    }
}
