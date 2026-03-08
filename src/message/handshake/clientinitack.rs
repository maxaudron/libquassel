use std::str::FromStr;

use crate::{
    HandshakeSerialize, ProtocolError, Result,
    primitive::{Variant, VariantList, VariantMap},
};

use super::Feature;

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
    /// Will only be available if Authenticators feature is on
    pub authenticators: Option<VariantList>,
    /// List of supported extended [`Feature`]
    pub feature_list: Vec<Feature>,
}

impl HandshakeSerialize for ClientInitAck {
    fn serialize(&self) -> Result<Vec<u8>> {
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

        if let Some(authenticators) = self.authenticators.as_ref() {
            values.insert(
                "Authenticators".to_string(),
                Variant::VariantList(authenticators.clone()),
            );
        }
        values.insert(
            "FeatureList".to_string(),
            Variant::StringList(self.feature_list.iter().map(|f| f.to_string()).collect()),
        );
        HandshakeSerialize::serialize(&values)
    }
}

impl TryFrom<VariantMap> for ClientInitAck {
    type Error = ProtocolError;

    fn try_from(input: VariantMap) -> Result<Self> {
        Ok(ClientInitAck {
            // TODO make this compatible with older clients
            core_features: 0,
            core_configured: input
                .get("Configured")
                .ok_or_else(|| ProtocolError::MissingField("Configured".to_string()))?
                .try_into()?,
            storage_backends: input
                .get("StorageBackends")
                .ok_or_else(|| ProtocolError::MissingField("StorageBackends".to_string()))?
                .try_into()?,
            authenticators: Some(
                input
                    .get("Authenticators")
                    .ok_or_else(|| ProtocolError::MissingField("Authenticators".to_string()))?
                    .try_into()?,
            ),
            feature_list: TryInto::<Vec<String>>::try_into(
                input
                    .get("FeatureList")
                    .ok_or_else(|| ProtocolError::MissingField("FeatureList".to_string()))?,
            )?
            .into_iter()
            .map(|s| Feature::from_str(&s))
            .collect::<Result<Vec<Feature>>>()?,
        })
    }
}
