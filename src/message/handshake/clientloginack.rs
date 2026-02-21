use crate::error::ProtocolError;
use crate::primitive::{Variant, VariantMap};
use crate::{HandshakeDeserialize, HandshakeSerialize};

/// ClientLoginAck is received after the client has successfully logged in
/// it has no fields
#[derive(Debug, Clone)]
pub struct ClientLoginAck;

impl HandshakeSerialize for ClientLoginAck {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut values: VariantMap = VariantMap::with_capacity(1);
        values.insert(
            "MsgType".to_string(),
            Variant::String("ClientLoginAck".to_string()),
        );
        return HandshakeSerialize::serialize(&values);
    }
}

impl HandshakeDeserialize for ClientLoginAck {
    fn parse(b: &[u8]) -> Result<(usize, Self), ProtocolError> {
        let (len, mut values): (usize, VariantMap) = HandshakeDeserialize::parse(b)?;

        let msgtype: String = values.remove("MsgType").unwrap().try_into().unwrap();

        if msgtype == "ClientLogin" {
            Ok((len, Self {}))
        } else {
            Err(ProtocolError::WrongMsgType)
        }
    }
}
