//! Generic Session handling implementation
//!
//! This module provides a relativly generic example implementation for handling the whole quassel session in
//! an application. This won't be usable in all cases but e.g. imediate mode GUIs can use it directly.

use std::collections::HashMap;

#[cfg(feature = "client")]
use crate::message::StatefulSyncableClient;
#[cfg(feature = "server")]
use crate::message::StatefulSyncableServer;

use log::{debug, error, warn};

use crate::{
    message::{
        objects::{Types, *},
        Class, InitData, SessionInit, SyncMessage, Syncable,
    },
    primitive::NetworkId,
};

/// Central struct that hold all our state
#[derive(Default, Debug)]
pub struct Session {
    pub alias_manager: AliasManager,
    pub buffer_syncer: BufferSyncer,
    pub backlog_manager: BacklogManager,
    pub buffer_view_manager: BufferViewManager,
    pub cert_manager: CertManager,
    pub core_info: CoreInfo,
    pub highlight_rule_manager: HighlightRuleManager,
    pub identities: Vec<Identity>,
    pub ignore_list_manager: IgnoreListManager,
    pub networks: HashMap<NetworkId, Network>,
    pub network_config: NetworkConfig,
}

/// The Session Trait is the main point of entry and implements the basic logic
///
/// Default implementations to handle sync and init of data structures is provided and you only have to
/// provide the getter functions.
pub trait SessionManager {
    fn alias_manager(&mut self) -> &mut AliasManager;
    fn buffer_syncer(&mut self) -> &mut BufferSyncer;
    fn backlog_manager(&mut self) -> &mut BacklogManager;
    fn buffer_view_manager(&mut self) -> &mut BufferViewManager;
    fn cert_manager(&mut self) -> &mut CertManager;
    fn core_info(&mut self) -> &mut CoreInfo;
    fn highlight_rule_manager(&mut self) -> &mut HighlightRuleManager;
    fn identities(&mut self) -> &mut Vec<Identity>;
    fn identity(&mut self, id: usize) -> Option<&mut Identity>;
    fn ignore_list_manager(&mut self) -> &mut IgnoreListManager;
    fn networks(&mut self) -> &mut HashMap<NetworkId, Network>;
    fn network(&mut self, id: i32) -> Option<&mut Network>;
    fn network_config(&mut self) -> &mut NetworkConfig;

    /// Handles an incoming [SyncMessage] and passes it on to the correct destination Object.
    fn sync(&mut self, msg: SyncMessage)
    where
        Self: Sized,
    {
        match msg.class_name {
            Class::AliasManager => self.alias_manager().sync(msg),
            Class::BacklogManager => self.backlog_manager().sync(msg),
            Class::BufferSyncer => self.buffer_syncer().sync(msg),
            Class::BufferViewConfig => (),
            Class::BufferViewManager => self.buffer_view_manager().sync(msg),
            Class::CoreInfo => self.core_info().sync(msg),
            // Synced via CoreInfo
            Class::CoreData => (),
            Class::HighlightRuleManager => self.highlight_rule_manager().sync(msg),
            Class::Identity => {
                let identity_id: i32 = msg.object_name.parse().unwrap();
                if let Some(identity) = self.identity(identity_id as usize) {
                    identity.sync(msg);
                } else {
                    warn!("could not find identity with id: {:?}", identity_id);
                }
            }
            Class::IgnoreListManager => self.ignore_list_manager().sync(msg),
            Class::CertManager => self.cert_manager().sync(msg),
            Class::Network => {
                let id: i32 = msg.object_name.parse().unwrap();
                if let Some(network) = self.network(id) {
                    network.sync(msg)
                }
            }
            // NetworkInfo does not receive SyncMessages directly but via Network
            Class::NetworkInfo => (),
            Class::NetworkConfig => match msg.object_name.as_ref() {
                "GlobalNetworkConfig" => self.network_config().sync(msg),
                _ => error!(
                    "received network config sync with unknown object name: {}",
                    msg.object_name
                ),
            },
            Class::IrcChannel => {
                let mut object_name = msg.object_name.split('/');
                let network_id: i32 = object_name.next().unwrap().parse().unwrap();
                let channel = object_name.next().unwrap();

                debug!("Syncing IrcChannel {} in Network {:?}", channel, network_id);

                if let Some(network) = self.network(network_id) {
                    if network.irc_channels.get_mut(channel).is_none() {
                        warn!(
                            "Could not find IrcChannel {} in Network {:?}",
                            channel, network_id
                        )
                    } else {
                        match msg.slot_name.as_str() {
                            "addChannelMode" => {
                                let mut msg = msg.clone();
                                let mode: char = get_param!(msg);
                                let mode_type: ChannelModeType = network.get_channel_mode_type(mode);

                                network.irc_channels.get_mut(channel).unwrap().add_channel_mode(
                                    mode_type,
                                    mode,
                                    get_param!(msg),
                                );
                            }
                            "removeChannelMode" => {
                                let mut msg = msg.clone();
                                let mode: char = get_param!(msg);
                                let mode_type: ChannelModeType = network.get_channel_mode_type(mode);

                                network
                                    .irc_channels
                                    .get_mut(channel)
                                    .unwrap()
                                    .remove_channel_mode(mode_type, mode, get_param!(msg));
                            }
                            _ => network.irc_channels.get_mut(channel).unwrap().sync(msg.clone()),
                        }
                    }
                } else {
                    warn!("Could not find Network {:?}", network_id)
                }
            }
            Class::IrcUser => {
                let mut object_name = msg.object_name.split('/');
                let network_id: i32 = object_name.next().unwrap().parse().unwrap();
                let user = object_name.next().unwrap();

                debug!("Syncing IrcUser {} in Network {:?}", user, network_id);

                if let Some(network) = self.network(network_id) {
                    if let Some(user) = network.irc_users.get_mut(user) {
                        user.sync(msg);
                        // TODO we probably need to deal with the quit here?
                        // and remove the user from the network
                    } else {
                        warn!("Could not find IrcUser {} in Network {:?}", user, network_id)
                    }
                } else {
                    warn!("Could not find Network {:?}", network_id)
                }
            }
            Class::Unknown => warn!("received unknown object syncmessage: {:?}", msg),
        }
    }

    fn session_init(&mut self, data: SessionInit) {
        *self.identities() = data.identities;
    }

    /// Handles an [InitData] messages and initializes the SessionManagers Objects.
    /// TODO handle automatic sending of InitRequest for whatever objects will need that.
    fn init(&mut self, data: InitData) {
        match data.init_data {
            Types::AliasManager(data) => self.alias_manager().init(data),
            Types::BufferSyncer(data) => self.buffer_syncer().init(data),
            Types::BufferViewConfig(data) => self.buffer_view_manager().init_buffer_view_config(data),
            Types::BufferViewManager(data) => self.buffer_view_manager().init(data),
            Types::CoreData(data) => self.core_info().set_core_data(data),
            Types::HighlightRuleManager(data) => self.highlight_rule_manager().init(data),
            Types::IgnoreListManager(data) => self.ignore_list_manager().init(data),
            Types::CertManager(data) => self.cert_manager().init(data),
            Types::Network(network) => {
                let id: NetworkId = NetworkId(data.object_name.parse().unwrap());
                self.networks().insert(id, network);
            }
            Types::NetworkInfo(_) => (),
            Types::NetworkConfig(_) => (),
            Types::IrcChannel(channel) => {
                let mut name = data.object_name.split("/");
                let id: i32 = name.next().unwrap().parse().unwrap();
                let name = name.next().unwrap();
                if let Some(network) = self.network(id) {
                    network.add_channel(name, channel)
                }
            }
            Types::Unknown(_) => (),
        }
    }
}

impl SessionManager for Session {
    fn alias_manager(&mut self) -> &mut AliasManager {
        &mut self.alias_manager
    }

    fn buffer_syncer(&mut self) -> &mut BufferSyncer {
        &mut self.buffer_syncer
    }

    fn backlog_manager(&mut self) -> &mut BacklogManager {
        &mut self.backlog_manager
    }

    fn buffer_view_manager(&mut self) -> &mut BufferViewManager {
        &mut self.buffer_view_manager
    }

    fn cert_manager(&mut self) -> &mut CertManager {
        &mut self.cert_manager
    }

    fn core_info(&mut self) -> &mut CoreInfo {
        &mut self.core_info
    }

    fn highlight_rule_manager(&mut self) -> &mut HighlightRuleManager {
        &mut self.highlight_rule_manager
    }

    fn identities(&mut self) -> &mut Vec<Identity> {
        &mut self.identities
    }

    fn identity(&mut self, id: usize) -> Option<&mut Identity> {
        self.identities.get_mut(id)
    }

    fn ignore_list_manager(&mut self) -> &mut IgnoreListManager {
        &mut self.ignore_list_manager
    }

    fn networks(&mut self) -> &mut HashMap<NetworkId, Network> {
        &mut self.networks
    }

    fn network(&mut self, id: i32) -> Option<&mut Network> {
        self.networks.get_mut(&NetworkId(id))
    }

    fn network_config(&mut self) -> &mut NetworkConfig {
        &mut self.network_config
    }
}
