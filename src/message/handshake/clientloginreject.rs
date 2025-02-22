use crate::error::ProtocolError;
use crate::primitive::{Variant, VariantMap};
use crate::HandshakeSerialize;

/// ClientLoginReject is received after the client failed to login
/// It contains an error message as String
#[derive(Debug, Clone)]
pub struct ClientLoginReject {
    pub error: String,
}

impl HandshakeSerialize for ClientLoginReject {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut values: VariantMap = VariantMap::with_capacity(1);
        values.insert(
            "MsgType".to_string(),
            Variant::String("ClientLoginReject".to_string()),
        );
        values.insert("ErrorString".to_string(), Variant::String(self.error.clone()));
        return HandshakeSerialize::serialize(&values);
    }
}

impl From<VariantMap> for ClientLoginReject {
    fn from(input: VariantMap) -> Self {
        ClientLoginReject {
            error: match_variant!(input.get("ErrorString").unwrap(), Variant::String),
        }
    }
}
