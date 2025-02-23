mod aliasmanager;
mod backlogmanager;
mod buffersyncer;
mod bufferviewconfig;
mod bufferviewmanager;
mod certmanager;
mod chanmodes;
mod coreinfo;
mod highlightrulemanager;
mod identity;
mod ignorelistmanager;
mod ircchannel;
mod ircuser;
mod network;
mod networkinfo;

pub use aliasmanager::*;
pub use backlogmanager::*;
pub use buffersyncer::*;
pub use bufferviewconfig::*;
pub use bufferviewmanager::*;
pub use certmanager::*;
pub use chanmodes::*;
pub use coreinfo::*;
pub use highlightrulemanager::*;
pub use identity::*;
pub use ignorelistmanager::*;
pub use ircchannel::*;
pub use ircuser::*;
pub use network::*;
pub use networkinfo::*;

use libquassel_derive::From;
use log::debug;

use super::{NetworkList, NetworkMap};
use crate::primitive::VariantList;

/// Central Enum containing and identifying all Quassel Protocol Types:
///
///  - [X] AliasManager
///  - [ ] BacklogManager
///  - [X] BufferSyncer
///  - [X] BufferViewConfig
///  - [X] BufferViewManager
///  - [X] CertManager
///  - [X] CoreInfo
///  - [X] CoreData
///  - [X] HighlightRuleManager
///  - [ ] Identity
///  - [X] IgnoreListManager
///  - [ ] IrcChannel
///  - [ ] IrcListHelper
///  - [ ] IrcUser
///  - [X] Network
///  - [X] NetworkInfo
///  - [X] NetworkConfig
// TODO Handle SyncedCoreInfo feature flag
#[derive(Debug, Clone, PartialEq, From)]
pub enum Types {
    AliasManager(AliasManager),
    BufferSyncer(BufferSyncer),
    BufferViewConfig(BufferViewConfig),
    BufferViewManager(BufferViewManager),
    // CoreInfo(CoreInfo),
    CoreData(CoreData),
    HighlightRuleManager(HighlightRuleManager),
    IgnoreListManager(IgnoreListManager),
    CertManager(CertManager),
    Network(network::Network),
    NetworkInfo(NetworkInfo),
    NetworkConfig(NetworkConfig),
    IrcChannel(IrcChannel),
    Unknown(VariantList),
}

impl Types {
    pub fn to_network(&self) -> VariantList {
        debug!("converting to network object: {:#?}", self);
        match self {
            Types::AliasManager(val) => val.to_network_list(),
            Types::BufferSyncer(val) => val.to_network_list(),
            Types::BufferViewConfig(val) => val.to_network_list(),
            Types::BufferViewManager(val) => val.to_network_list(),
            // Types::CoreInfo(val) => vec![val.to_network_map().into()],
            Types::CoreData(val) => vec![val.to_network_map().into()],
            Types::HighlightRuleManager(val) => val.to_network_list(),
            Types::IgnoreListManager(val) => val.to_network_list(),
            Types::CertManager(val) => val.to_network_list(),
            Types::Network(val) => val.to_network_list(),
            Types::NetworkInfo(val) => val.to_network_list(),
            Types::NetworkConfig(val) => val.to_network_list(),
            Types::IrcChannel(val) => val.to_network_list(),
            Types::Unknown(val) => val.clone(),
        }
    }

    pub fn from_network(class_name: &str, object_name: &str, input: &mut VariantList) -> Self {
        debug!("converting {} from network object: {:#?}", class_name, input);
        match class_name {
            "AliasManager" => Types::AliasManager(AliasManager::from_network_list(input)),
            "BufferSyncer" => Types::BufferSyncer(BufferSyncer::from_network_list(input)),
            "BufferViewConfig" => {
                let mut config = BufferViewConfig::from_network_list(input);
                config.buffer_view_id = object_name.parse().unwrap();
                Types::BufferViewConfig(config)
            }
            "BufferViewManager" => Types::BufferViewManager(BufferViewManager::from_network_list(input)),
            // "CoreInfo" => Types::CoreInfo(CoreInfo::from_network_map(
            //     &mut input.remove(0).try_into().unwrap(),
            // )),
            "CoreData" => Types::CoreData(CoreData::from_network_map(
                &mut input.remove(0).try_into().unwrap(),
            )),
            "HighlightRuleManager" => {
                Types::HighlightRuleManager(HighlightRuleManager::from_network_list(input))
            }
            "IgnoreListManager" => Types::IgnoreListManager(IgnoreListManager::from_network_list(input)),
            "CertManager" => Types::CertManager(CertManager::from_network_list(input)),
            "Network" => Types::Network(Network::from_network_list(input)),
            "NetworkInfo" => Types::NetworkInfo(NetworkInfo::from_network_list(input)),
            "NetworkConfig" => Types::NetworkConfig(NetworkConfig::from_network_list(input)),
            "IrcChannel" => Types::IrcChannel(IrcChannel::from_network_list(input)),
            _ => Types::Unknown(input.to_owned()),
        }
    }
}
