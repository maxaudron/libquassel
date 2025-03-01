use crate::primitive::{PeerPtr, Variant};

use super::{Direction, RpcCallType};

#[derive(Clone, Debug, PartialEq)]
pub struct ChangePassword {
    /// Always zero, only has a value within of the core itself.
    peer: PeerPtr,
    /// Username
    user: String,
    /// Old Password
    before: String,
    // New Password
    after: String,
}

impl RpcCallType for ChangePassword {
    const NAME: &str = "2changePassword(PeerPtr,QString,QString,QString)";
    const DIRECTION: Direction = Direction::ClientToServer;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            self.peer.clone().into(),
            self.user.clone().into(),
            self.before.clone().into(),
            self.after.clone().into(),
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
                peer: match_variant!(input.remove(0), Variant::PeerPtr),
                user: match_variant!(input.remove(0), Variant::String),
                before: match_variant!(input.remove(0), Variant::String),
                after: match_variant!(input.remove(0), Variant::String),
            }
            .into(),
        ))
    }
}

/// Returns if the recent password change attempt has been a success.
/// This is one of the few responses which only gets sent to the client which sent the original request.
#[derive(Clone, Debug, PartialEq)]
pub struct PasswordChanged {
    /// Always zero, only has a value within of the core itself.
    peer: PeerPtr,
    success: bool,
}

impl RpcCallType for PasswordChanged {
    const NAME: &str = "2passwordChanged(PeerPtr,bool)";
    const DIRECTION: Direction = Direction::ServerToClient;

    fn to_network(&self) -> Result<Vec<crate::primitive::Variant>, crate::ProtocolError> {
        Ok(vec![
            Variant::ByteArray(Self::NAME.to_string()),
            self.peer.clone().into(),
            self.success.clone().into(),
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
                peer: match_variant!(input.remove(0), Variant::PeerPtr),
                success: match_variant!(input.remove(0), Variant::bool),
            }
            .into(),
        ))
    }
}
