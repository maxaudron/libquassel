use libquassel_derive::{NetworkList, NetworkMap};

use crate::message::signalproxy::translation::NetworkMap;
use crate::message::{Class, Syncable};
use crate::primitive::{DateTime, StringList};

#[derive(Default, Debug, Clone, PartialEq, NetworkList, NetworkMap)]
#[network(repr = "map")]
pub struct CoreInfo {
    #[network(rename = "coreData", variant = "VariantMap", network = "map")]
    pub core_data: CoreData,
}

impl CoreInfo {
    pub fn set_core_data(&mut self, data: CoreData) {
        #[cfg(feature = "server")]
        libquassel_derive::sync!("setCoreData", [data.to_network_map()]);

        self.core_data = data;
    }
}

#[cfg(feature = "client")]
impl crate::message::StatefulSyncableClient for CoreInfo {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        #[allow(clippy::single_match)]
        match msg.slot_name.as_str() {
            "setCoreData" => self.set_core_data(CoreData::from_network_map(&mut get_param!(msg))),
            _ => (),
        }
    }

    /// Not Implemented
    fn request_update(&mut self)
    where
        Self: Sized,
    {
    }
}

#[cfg(feature = "server")]
impl crate::message::StatefulSyncableServer for CoreInfo {
    /// Not Implemented
    fn request_update(&mut self, mut _param: <CoreInfo as NetworkMap>::Item)
    where
        Self: Sized,
    {
    }
}

impl Syncable for CoreInfo {
    const CLASS: Class = Class::CoreInfo;
}

#[derive(Debug, Clone, PartialEq, NetworkMap)]
#[network(repr = "map")]
pub struct CoreData {
    #[network(rename = "quasselVersion")]
    pub quassel_version: String,
    #[network(rename = "quasselBuildDate")]
    pub quassel_build_date: String,
    #[network(rename = "startTime")]
    pub start_time: DateTime,
    #[network(rename = "sessionConnectedClients")]
    pub session_connected_clients: i32,
    #[network(
        rename = "sessionConnectedClientData",
        variant = "VariantList",
        network = "map"
    )]
    pub session_connected_client_data: Vec<ConnectedClient>,
}

impl Default for CoreData {
    fn default() -> Self {
        Self {
            quassel_version: Default::default(),
            quassel_build_date: Default::default(),
            start_time: DateTime::UNIX_EPOCH,
            session_connected_clients: Default::default(),
            session_connected_client_data: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, NetworkMap)]
#[network(repr = "map")]
pub struct ConnectedClient {
    #[network(rename = "id")]
    pub id: i32,
    #[network(rename = "remoteAddress")]
    pub remote_address: String,
    // #[network(rename = "location")]
    // location: String,
    #[network(rename = "clientVersion")]
    pub client_version: String,
    #[network(rename = "clientVersionDate")]
    pub client_version_date: String,
    #[network(rename = "connectedSince")]
    pub connected_since: DateTime,
    #[network(rename = "secure")]
    pub secure: bool,
    #[network(rename = "features")]
    pub features: u32,
    #[network(rename = "featureList")]
    pub feature_list: StringList,
}
