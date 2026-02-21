use crate::error::ProtocolError;
use crate::primitive::{Variant, VariantMap};
use crate::HandshakeSerialize;

/// ClientInitReject is received when the initialization fails
#[derive(Debug, Clone)]
pub struct ClientInitReject {
    /// String with an error message of what went wrong
    pub error: String,
}

impl HandshakeSerialize for ClientInitReject {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut values: VariantMap = VariantMap::with_capacity(2);
        values.insert(
            "MsgType".to_string(),
            Variant::String("ClientInitReject".to_string()),
        );
        values.insert("ErrorString".to_string(), Variant::String(self.error.clone()));
        HandshakeSerialize::serialize(&values)
    }
}

impl TryFrom<VariantMap> for ClientInitReject {
    type Error = ProtocolError;

    fn try_from(mut input: VariantMap) -> Result<Self, Self::Error> {
        Ok(ClientInitReject {
            error: input
                .remove("ErrorString")
                .ok_or_else(|| ProtocolError::MissingField("ErrorString".to_string()))?
                .try_into()?,
        })
    }
}
