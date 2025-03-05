use libquassel_derive::{NetworkList, NetworkMap};

use crate::message::signalproxy::translation::NetworkMap;
use crate::message::{Class, Syncable};
use crate::primitive::{DateTime, StringList};
use crate::Result;

/// Metadata about the Core a client is connected to
#[derive(Default, Debug, Clone, PartialEq, NetworkList, NetworkMap)]
#[network(repr = "map")]
pub struct CoreInfo {
    #[network(rename = "coreData", variant = "VariantMap", network = "map")]
    pub core_data: CoreData,
}

impl CoreInfo {
    pub fn set_core_data(&mut self, data: CoreData) -> Result<()> {
        #[cfg(feature = "server")]
        libquassel_derive::sync!("setCoreData", [data.to_network_map()?])?;

        self.core_data = data;

        Ok(())
    }
}

#[cfg(feature = "client")]
impl crate::message::StatefulSyncableClient for CoreInfo {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage) -> Result<()>
    where
        Self: Sized,
    {
        #[allow(clippy::single_match)]
        match msg.slot_name.as_str() {
            "setCoreData" => self.set_core_data(CoreData::from_network_map(&mut get_param!(msg))?),
            unknown => Err(crate::ProtocolError::UnknownMsgSlotName(unknown.to_string())),
        }
    }

    /// Not Implemented
    fn request_update(&mut self) -> crate::Result<()>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

#[cfg(feature = "server")]
impl crate::message::StatefulSyncableServer for CoreInfo {
    /// Not Implemented
    fn request_update(&mut self, mut _param: <CoreInfo as NetworkMap>::Item) -> Result<()>
    where
        Self: Sized,
    {
        Ok(())
    }
}

impl Syncable for CoreInfo {
    const CLASS: Class = Class::CoreInfo;
}

/// Metadata about the Core a client is connected to
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
