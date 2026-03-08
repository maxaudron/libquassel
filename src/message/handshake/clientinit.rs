use std::str::FromStr;

use crate::{
    HandshakeSerialize, ProtocolError, Result,
    message::Feature,
    primitive::{Variant, VariantMap},
};

/// ClientInit is the Initial message send to the core after establishing a base layer comunication.
#[derive(Debug, Clone)]
pub struct ClientInit {
    /// Version of the client
    pub client_version: String,
    /// Build date of the client
    pub client_date: String,
    /// supported features as bitflags
    pub client_features: u32,
    /// List of supported extended [`Feature`]
    pub feature_list: Vec<Feature>,
}

impl HandshakeSerialize for ClientInit {
    fn serialize(&self) -> Result<Vec<u8>> {
        let mut values: VariantMap = VariantMap::with_capacity(5);
        values.insert("MsgType".to_string(), Variant::String("ClientInit".to_string()));
        values.insert(
            "ClientVersion".to_string(),
            Variant::String(self.client_version.clone()),
        );
        values.insert(
            "ClientDate".to_string(),
            Variant::String(self.client_date.clone()),
        );
        values.insert("Features".to_string(), Variant::u32(self.client_features));
        values.insert(
            "FeatureList".to_string(),
            Variant::StringList(self.feature_list.iter().map(|f| f.to_string()).collect()),
        );
        HandshakeSerialize::serialize(&values)
    }
}

impl TryFrom<VariantMap> for ClientInit {
    type Error = ProtocolError;

    fn try_from(mut input: VariantMap) -> Result<Self> {
        Ok(ClientInit {
            client_version: input
                .remove("ClientVersion")
                .ok_or_else(|| ProtocolError::MissingField("ClientVersion".to_string()))?
                .try_into()?,
            client_date: input
                .remove("ClientDate")
                .ok_or_else(|| ProtocolError::MissingField("ClientDate".to_string()))?
                .try_into()?,
            client_features: input
                .remove("Features")
                .ok_or_else(|| ProtocolError::MissingField("Features".to_string()))?
                .try_into()?,
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
