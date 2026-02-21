use crate::{
    message::objects::Identity,
    primitive::{IdentityId, Variant, VariantMap},
};

use super::{Direction, RpcCallType};

#[derive(Clone, Debug, PartialEq)]
pub struct CreateIdentity {
    identity: Identity,
    /// Always Empty
    additional: VariantMap,
}

impl RpcCallType for CreateIdentity {
    const NAME: &str = "2createIdentity(Identity,QVariantMap)";
    const DIRECTION: Direction = Direction::ClientToServer;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            self.identity.clone().into(),
            self.additional.clone().into(),
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
            CreateIdentity {
                identity: input.remove(0).try_into().unwrap(),
                additional: input.remove(0).try_into().unwrap(),
            }
            .into(),
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RemoveIdentity {
    identity_id: IdentityId,
}

impl RpcCallType for RemoveIdentity {
    const NAME: &str = "2removeIdentity(IdentityId)";
    const DIRECTION: Direction = Direction::ClientToServer;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            Variant::IdentityId(self.identity_id.clone()),
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
                identity_id: input.remove(0).try_into().unwrap(),
            }
            .into(),
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdentityCreated {
    identity: Identity,
}

impl RpcCallType for IdentityCreated {
    const NAME: &str = "2identityCreated(Identity)";
    const DIRECTION: Direction = Direction::ServerToClient;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            self.identity.clone().into(),
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
            IdentityCreated {
                identity: input.remove(0).try_into().unwrap(),
            }
            .into(),
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdentityRemoved {
    identity_id: IdentityId,
}

impl RpcCallType for IdentityRemoved {
    const NAME: &str = "2identityRemoved(IdentityId)";
    const DIRECTION: Direction = Direction::ServerToClient;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            Variant::IdentityId(self.identity_id.clone()),
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
                identity_id: input.remove(0).try_into().unwrap(),
            }
            .into(),
        ))
    }
}
