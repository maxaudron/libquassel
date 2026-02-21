use crate::error::ProtocolError;
use crate::primitive::{Variant, VariantMap};
use crate::HandshakeSerialize;

/// Login to the core with user data
/// username and password are transmitted in plain text
#[derive(Debug, Clone)]
pub struct ClientLogin {
    pub user: String,
    pub password: String,
}

impl HandshakeSerialize for ClientLogin {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut values: VariantMap = VariantMap::new();
        values.insert("MsgType".to_string(), Variant::String("ClientLogin".to_string()));
        values.insert("User".to_string(), Variant::String(self.user.clone()));
        values.insert("Password".to_string(), Variant::String(self.password.clone()));
        HandshakeSerialize::serialize(&values)
    }
}

impl TryFrom<VariantMap> for ClientLogin {
    type Error = ProtocolError;

    fn try_from(mut input: VariantMap) -> Result<Self, Self::Error> {
        Ok(ClientLogin {
            user: input
                .remove("User")
                .ok_or_else(|| ProtocolError::MissingField("User".to_string()))?
                .try_into()?,
            password: input
                .remove("Password")
                .ok_or_else(|| ProtocolError::MissingField("Password".to_string()))?
                .try_into()?,
        })
    }
}
