use crate::error::ProtocolError;
use crate::primitive::{Variant, VariantList, VariantMap};
use crate::HandshakeSerialize;

/// ClientInitAck is received when the initialization was successfull
#[derive(Debug, Clone)]
pub struct ClientInitAck {
    /// Flags of supported legacy features
    pub core_features: u32,
    /// If the core has already been configured
    pub core_configured: bool,
    /// List of VariantMaps of info on available backends
    pub storage_backends: VariantList,
    /// List of VariantMaps of info on available authenticators
    #[cfg(feature = "authenticators")]
    #[cfg_attr(docsrs, doc(cfg(feature = "authenticators")))]
    pub authenticators: VariantList,
    /// List of supported extended features
    pub feature_list: Vec<String>,
}

impl HandshakeSerialize for ClientInitAck {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut values: VariantMap = VariantMap::with_capacity(6);
        values.insert(
            "MsgType".to_string(),
            Variant::String("ClientInitAck".to_string()),
        );
        values.insert("CoreFeatures".to_string(), Variant::u32(self.core_features));
        values.insert("Configured".to_string(), Variant::bool(self.core_configured));
        values.insert(
            "StorageBackends".to_string(),
            Variant::VariantList(self.storage_backends.clone()),
        );
        #[cfg(feature = "authenticators")]
        values.insert(
            "Authenticators".to_string(),
            Variant::VariantList(self.authenticators.clone()),
        );
        values.insert(
            "FeatureList".to_string(),
            Variant::StringList(self.feature_list.clone()),
        );
        return HandshakeSerialize::serialize(&values);
    }
}

impl From<VariantMap> for ClientInitAck {
    fn from(input: VariantMap) -> Self {
        ClientInitAck {
            // TODO make this compatible with older clients
            core_features: 0,
            core_configured: match_variant!(input.get("Configured").unwrap(), Variant::bool),
            storage_backends: match_variant!(input.get("StorageBackends").unwrap(), Variant::VariantList),
            #[cfg(feature = "authenticators")]
            authenticators: match_variant!(input.get("Authenticators").unwrap(), Variant::VariantList),
            feature_list: match_variant!(input.get("FeatureList").unwrap(), Variant::StringList),
        }
    }
}
