use crate::error::ProtocolError;
use crate::message::objects::Identity;
use crate::primitive::{BufferInfo, NetworkId, Variant, VariantList, VariantMap};
use crate::HandshakeSerialize;

/// SessionInit is received along with ClientLoginAck to initialize that user Session
// TODO Replace with proper types
#[derive(Debug, Clone)]
pub struct SessionInit {
    /// List of all configured identities
    pub identities: Vec<Identity>,
    /// List of all existing buffers
    pub buffers: Vec<BufferInfo>, // Vec<Variant::BufferInfo()>
    /// Ids of all networks
    pub network_ids: Vec<NetworkId>,
}

impl From<VariantMap> for SessionInit {
    fn from(input: VariantMap) -> Self {
        let mut state: VariantMap = input.get("SessionState").unwrap().try_into().unwrap();

        log::trace!("sessionstate: {:#?}", state);

        let identities: VariantList = state.remove("Identities").unwrap().try_into().unwrap();
        let buffers: VariantList = state.remove("BufferInfos").unwrap().try_into().unwrap();
        let network_ids: VariantList = state.remove("NetworkIds").unwrap().try_into().unwrap();

        SessionInit {
            identities: identities.into_iter().map(|x| x.try_into().unwrap()).collect(),
            buffers: buffers
                .iter()
                .map(|buffer| match buffer {
                    Variant::BufferInfo(buffer) => buffer.clone(),
                    _ => unimplemented!(),
                })
                .collect(),
            network_ids: network_ids
                .iter()
                .map(|network| match network {
                    Variant::NetworkId(network) => network.clone(),
                    _ => unimplemented!(),
                })
                .collect(),
        }
    }
}

impl HandshakeSerialize for SessionInit {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut values: VariantMap = VariantMap::with_capacity(4);
        values.insert("MsgType".to_string(), Variant::String("SessionInit".to_string()));
        // values.insert(
        //     "Identities".to_string(),
        //     Variant::VariantList(
        //         self.identities
        //             .iter()
        //             .map(|ident| Variant::VariantMap(ident.clone().into()))
        //             .collect(),
        //     ),
        // );
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
                    .map(|id| Variant::NetworkId(id.clone()))
                    .collect(),
            ),
        );
        return HandshakeSerialize::serialize(&values);
    }
}
