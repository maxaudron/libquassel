mod aliasmanager;
mod buffersyncer;
mod bufferview;
mod certmanager;
mod coreinfo;
mod highlightrulemanager;
mod identity;
mod ignorelistmanager;
mod ircchannel;
mod ircuser;
mod network;
mod networkinfo;

use std::convert::TryInto;

pub use aliasmanager::*;
pub use buffersyncer::*;
pub use bufferview::*;
pub use certmanager::*;
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

use super::{Network, NetworkList, NetworkMap};
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
#[derive(Debug, Clone, PartialEq, From)]
pub enum Types {
    AliasManager(AliasManager),
    BufferSyncer(BufferSyncer),
    BufferViewConfig(BufferViewConfig),
    BufferViewManager(BufferViewManager),
    CoreInfo(CoreInfo),
    CoreData(CoreData),
    HighlightRuleManager(HighlightRuleManager),
    IgnoreListManager(IgnoreListManager),
    CertManager(CertManager),
    Network(network::Network),
    NetworkInfo(NetworkInfo),
    NetworkConfig(NetworkConfig),
    Unknown(VariantList),
}

impl Types {
    pub fn to_network(&self) -> VariantList {
        debug!("converting to network object: {:#?}", self);
        match self {
            Types::AliasManager(val) => val.to_network_list(),
            Types::BufferSyncer(val) => val.to_network(),
            Types::BufferViewConfig(val) => val.to_network(),
            Types::BufferViewManager(val) => val.to_network(),
            Types::CoreInfo(val) => vec![val.to_network().into()],
            Types::CoreData(val) => vec![val.to_network().into()],
            Types::HighlightRuleManager(val) => val.to_network(),
            Types::IgnoreListManager(val) => val.to_network(),
            Types::CertManager(val) => val.to_network(),
            Types::Network(val) => val.to_network(),
            Types::NetworkInfo(val) => val.to_network(),
            Types::NetworkConfig(val) => val.to_network(),
            Types::Unknown(val) => val.clone(),
        }
    }

    pub fn from_network(class_name: &str, input: &mut VariantList) -> Self {
        debug!(
            "converting {} from network object: {:#?}",
            class_name, input
        );
        match class_name {
            "AliasManager" => Types::AliasManager(AliasManager::from_network_list(input)),
            "BufferSyncer" => Types::BufferSyncer(BufferSyncer::from_network(input)),
            "BufferViewConfig" => Types::BufferViewConfig(BufferViewConfig::from_network(input)),
            "BufferViewManager" => Types::BufferViewManager(BufferViewManager::from_network(input)),
            "CoreInfo" => Types::CoreInfo(CoreInfo::from_network(
                &mut input.remove(0).try_into().unwrap(),
            )),
            "CoreData" => Types::CoreData(CoreData::from_network(
                &mut input.remove(0).try_into().unwrap(),
            )),
            "HighlightRuleManager" => {
                Types::HighlightRuleManager(HighlightRuleManager::from_network(input))
            }
            "IgnoreListManager" => Types::IgnoreListManager(IgnoreListManager::from_network(input)),
            "CertManager" => Types::CertManager(CertManager::from_network(input)),
            "Network" => Types::Network(Network::from_network(input)),
            "NetworkInfo" => Types::NetworkInfo(NetworkInfo::from_network(input)),
            "NetworkConfig" => Types::NetworkConfig(NetworkConfig::from_network(input)),
            _ => Types::Unknown(input.to_owned()),
        }
    }
}
