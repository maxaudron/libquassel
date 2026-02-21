mod clientinit;
mod clientinitack;
mod clientinitreject;
mod clientlogin;
mod clientloginack;
mod clientloginreject;
mod connack;
mod features;
mod init;
mod protocol;
mod sessioninit;
mod types;

pub use clientinit::*;
pub use clientinitack::*;
pub use clientinitreject::*;
pub use clientlogin::*;
pub use clientloginack::*;
pub use clientloginreject::*;
pub use connack::*;
pub use features::*;
pub use init::*;
pub use protocol::*;
pub use sessioninit::*;
#[allow(unused_imports)]
pub use types::*;

use crate::error::ProtocolError;
use crate::primitive::VariantMap;
use crate::{HandshakeDeserialize, HandshakeSerialize};

#[derive(Debug, Clone)]
pub enum HandshakeMessage {
    ClientInit(ClientInit),
    ClientInitAck(ClientInitAck),
    ClientInitReject(ClientInitReject),
    ClientLogin(ClientLogin),
    ClientLoginAck,
    ClientLoginReject(ClientLoginReject),
    SessionInit(SessionInit),
}

impl HandshakeSerialize for HandshakeMessage {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        match self {
            HandshakeMessage::ClientInit(inner) => inner.serialize(),
            HandshakeMessage::ClientInitAck(inner) => inner.serialize(),
            HandshakeMessage::ClientInitReject(inner) => inner.serialize(),
            HandshakeMessage::ClientLogin(inner) => inner.serialize(),
            HandshakeMessage::ClientLoginAck => ClientLoginAck.serialize(),
            HandshakeMessage::ClientLoginReject(inner) => inner.serialize(),
            HandshakeMessage::SessionInit(inner) => inner.serialize(),
        }
    }
}

impl HandshakeDeserialize for HandshakeMessage {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let (size, mut res) = VariantMap::parse(b)?;

        let msgtype: String = res.remove("MsgType").unwrap().try_into().unwrap();
        match msgtype.as_str() {
            "ClientInit" => Ok((size, HandshakeMessage::ClientInit(res.into()))),
            "ClientInitAck" => Ok((size, HandshakeMessage::ClientInitAck(res.into()))),
            "ClientInitReject" => Ok((size, HandshakeMessage::ClientInitReject(res.into()))),
            "ClientLogin" => Ok((size, HandshakeMessage::ClientLogin(res.into()))),
            "ClientLoginAck" => Ok((size, HandshakeMessage::ClientLoginAck)),
            "ClientLoginReject" => Ok((size, HandshakeMessage::ClientLoginReject(res.into()))),
            "SessionInit" => Ok((size, HandshakeMessage::SessionInit(res.into()))),
            _ => unimplemented!(),
        }
    }
}
