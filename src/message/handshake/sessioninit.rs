use crate::error::{ProtocolError, Result};
use crate::message::objects::Identity;
use crate::message::NetworkMap;
use crate::primitive::{BufferInfo, NetworkId, Variant, VariantList, VariantMap};
use crate::HandshakeSerialize;

/// SessionInit is received along with ClientLoginAck to initialize that user Session
/// Upon receiving this the client needs to send [InitRequest] for the Networks using the NetworkId
#[derive(Debug, Clone)]
pub struct SessionInit {
    /// List of all configured identities
    pub identities: Vec<Identity>,
    /// List of all existing buffers
    pub buffers: Vec<BufferInfo>, // Vec<Variant::BufferInfo()>
    /// Ids of all networks
    pub network_ids: Vec<NetworkId>,
}

impl TryFrom<VariantMap> for SessionInit {
    type Error = ProtocolError;

    fn try_from(input: VariantMap) -> Result<Self> {
        let mut state: VariantMap = input
            .get("SessionState")
            .ok_or_else(|| ProtocolError::MissingField("SessionState".to_string()))?
            .try_into()?;

        log::trace!("sessionstate: {:#?}", state);

        let identities: VariantList = state
            .remove("Identities")
            .ok_or_else(|| ProtocolError::MissingField("Identities".to_string()))?
            .try_into()?;
        let buffers: VariantList = state
            .remove("BufferInfos")
            .ok_or_else(|| ProtocolError::MissingField("BufferInfos".to_string()))?
            .try_into()?;
        let network_ids: VariantList = state
            .remove("NetworkIds")
            .ok_or_else(|| ProtocolError::MissingField("NetworkIds".to_string()))?
            .try_into()?;

        Ok(SessionInit {
            identities: identities
                .into_iter()
                .map(|x| x.try_into())
                .collect::<Result<Vec<_>>>()?,
            buffers: buffers
                .iter()
                .map(|buffer| match buffer {
                    Variant::BufferInfo(buffer) => Ok(buffer.clone()),
                    _ => Err(ProtocolError::WrongVariant),
                })
                .collect::<Result<Vec<_>>>()?,
            network_ids: network_ids
                .iter()
                .map(|network| match network {
                    Variant::NetworkId(network) => Ok(*network),
                    _ => Err(ProtocolError::WrongVariant),
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

impl HandshakeSerialize for SessionInit {
    fn serialize(&self) -> Result<Vec<u8>> {
        let mut values: VariantMap = VariantMap::with_capacity(4);
        values.insert("MsgType".to_string(), Variant::String("SessionInit".to_string()));
        values.insert(
            "Identities".to_string(),
            Variant::VariantList(
                self.identities
                    .iter()
                    .map(|ident| -> Result<Variant> { Ok(Variant::VariantMap(ident.to_network_map()?)) })
                    .collect::<Result<Vec<Variant>>>()?,
            ),
        );
        values.insert(
            "BufferInfos".to_string(),
            Variant::VariantList(
                self.buffers
                    .iter()
                    .map(|buffer| Variant::BufferInfo(buffer.clone()))
                    .collect(),
            ),
        );
        values.insert(
            "NetworkIds".to_string(),
            Variant::VariantList(
                self.network_ids
                    .iter()
                    .map(|id| Variant::NetworkId(*id))
                    .collect(),
            ),
        );
        HandshakeSerialize::serialize(&values)
    }
}
