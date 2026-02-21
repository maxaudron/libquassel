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

impl From<VariantMap> for ClientLogin {
    fn from(mut input: VariantMap) -> Self {
        ClientLogin {
            user: input.remove("User").unwrap().try_into().unwrap(),
            password: input.remove("Password").unwrap().try_into().unwrap(),
        }
    }
}
