use std::collections::HashMap;

use itertools::Itertools;
use log::{error, warn};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use libquassel_derive::{sync, NetworkMap, Setters};

use crate::error::ProtocolError;
use crate::message::signalproxy::translation::NetworkMap;
use crate::message::{Class, NetworkList, Syncable};
use crate::primitive::{Variant, VariantList, VariantMap};
use crate::serialize::{Deserialize, Serialize, UserType};

use super::{ircchannel::IrcChannel, ircuser::IrcUser, networkinfo::NetworkInfo};

#[derive(Default, Debug, Clone, PartialEq, Setters)]
pub struct Network {
    pub my_nick: String,
    pub latency: i32,
    pub current_server: String,
    #[setter(name = "connected")]
    pub is_connected: bool,
    pub connection_state: ConnectionState,
    #[setter(skip)]
    pub prefixes: Vec<char>,
    #[setter(skip)]
    pub prefix_modes: Vec<char>,
    #[setter(skip)]
    pub channel_modes: HashMap<ChannelModeType, String>,
    #[setter(skip)]
    pub irc_users: HashMap<String, IrcUser>,
    #[setter(skip)]
    pub irc_channels: HashMap<String, IrcChannel>,
    #[setter(skip)]
    pub supports: HashMap<String, String>,
    #[setter(skip)]
    pub caps: HashMap<String, String>,
    #[setter(skip)]
    pub caps_enabled: Vec<String>,
    #[setter(skip)]
    pub network_info: NetworkInfo,
}

impl Network {
    pub fn get_channel_mode_type(&self, mode: char) -> ChannelModeType {
        if let Some((mode_type, _)) = self.channel_modes.iter().find(|(_, v)| v.contains(mode)) {
            *mode_type
        } else {
            ChannelModeType::NotAChanmode
        }
    }

    /// The `channel_modes` field is populated by the ``supports["CHANMODES"]` string,
    /// which is represented as the channel mode types a,b,c,d in a comma sepperated string.
    fn determine_channel_mode_types(&mut self) {
        let mut modes: Vec<&str> = self.supports.get("CHANMODES").unwrap().split(',').collect();

        self.channel_modes
            .insert(ChannelModeType::DChanmode, modes.pop().unwrap().to_owned());
        self.channel_modes
            .insert(ChannelModeType::CChanmode, modes.pop().unwrap().to_owned());
        self.channel_modes
            .insert(ChannelModeType::BChanmode, modes.pop().unwrap().to_owned());
        self.channel_modes
            .insert(ChannelModeType::AChanmode, modes.pop().unwrap().to_owned());
    }

    fn determine_prefixes(&mut self) {
        let default_prefixes = vec!['~', '&', '@', '%', '+'];
        let default_prefix_modes = vec!['q', 'a', 'o', 'h', 'v'];

        match self.supports.get("PREFIX") {
            Some(prefix) => {
                if prefix.starts_with('(') {
                    let (prefix_modes, prefixes) = prefix[1..].split_once(')').unwrap();

                    self.prefix_modes = prefix_modes.chars().collect();
                    self.prefixes = prefixes.chars().collect();
                } else {
                    self.prefixes = default_prefixes;
                    self.prefix_modes = default_prefix_modes;
                }
            }
            None => {
                self.prefixes = default_prefixes;
                self.prefix_modes = default_prefix_modes;
            }
        }
    }

    pub fn add_channel(&mut self, name: &str, channel: IrcChannel) {
        self.irc_channels.insert(name.to_owned(), channel);
    }

    pub fn connect(&self) {
        #[cfg(feature = "client")]
        sync!("requestConnect", [])
    }

    pub fn disconnect(&self) {
        #[cfg(feature = "client")]
        sync!("requestDisconnect", [])
    }

    pub fn set_network_info(&mut self, network_info: NetworkInfo) {
        #[cfg(feature = "client")]
        sync!("requestSetNetworkInfo", [network_info.to_network_map()]);

        self.network_info = network_info;
    }

    /// Enable the capability `cap` if it is not already enabled
    pub fn acknowledge_cap(&mut self, cap: String) {
        #[cfg(feature = "server")]
        sync!("acknowledgeCap", [cap.clone()]);

        if !self.caps_enabled.contains(&cap) {
            self.caps_enabled.push(cap);
        } else {
            warn!("Capability {} already enabled", cap)
        }
    }

    /// Add a new capability supported by the server
    pub fn add_cap(&mut self, cap: String, value: String) {
        #[cfg(feature = "server")]
        sync!("addCap", [cap.clone(), value.clone()]);

        self.caps.insert(cap, value);
    }

    /// Clear `caps` and `caps_enabled`
    pub fn clear_caps(&mut self) {
        #[cfg(feature = "server")]
        sync!("clearCaps", []);

        self.caps.clear();
        self.caps_enabled.clear();
    }

    /// Remove a capability from `caps` and `caps_enabled`
    pub fn remove_cap(&mut self, cap: String) {
        #[cfg(feature = "server")]
        sync!("removeCap", [cap.clone()]);

        self.caps.remove(&cap);
        if let Some((i, _)) = self.caps_enabled.iter().find_position(|c| **c == cap) {
            self.caps_enabled.remove(i);
        }
    }

    // TODO
    pub fn add_irc_channel(&mut self, _name: String) {}
    pub fn add_irc_user(&mut self, _hostmask: String) {}

    pub fn add_support(&mut self, key: String, value: String) {
        #[cfg(feature = "server")]
        sync!("addSupport", [key.clone(), value.clone()]);

        self.supports.insert(key, value);
    }

    pub fn remove_support(&mut self, key: String) {
        #[cfg(feature = "server")]
        sync!("removeSupport", [key.clone()]);

        self.supports.remove(&key);
    }

    pub fn emit_connection_error(&mut self, error: String) {
        #[cfg(feature = "server")]
        sync!("emitConnectionError", [error.clone()]);

        error!("{}", error)
    }

    /// Rename the user object in the network object
    /// TODO the actual nick change is done with a sepperate sync message against the IrcUser object?
    pub fn irc_user_nick_changed(&mut self, before: String, after: String) {
        #[cfg(feature = "server")]
        sync!("ircUserNickChanged", [before.clone(), after.clone()]);

        if let Some(user) = self.irc_users.remove(&before) {
            self.irc_users.insert(after, user);
        } else {
            warn!("irc user {} not found", before);
        }
    }
}

impl Syncable for Network {
    const CLASS: Class = Class::Network;
}

#[cfg(feature = "client")]
impl crate::message::StatefulSyncableClient for Network {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "acknowledgeCap" => self.acknowledge_cap(get_param!(msg)),
            "addCap" => self.add_cap(get_param!(msg), get_param!(msg)),
            "addIrcChannel" => self.add_irc_channel(get_param!(msg)),
            "addIrcUser" => self.add_irc_user(get_param!(msg)),
            "addSupport" => self.add_support(get_param!(msg), get_param!(msg)),
            "clearCaps" => self.clear_caps(),
            "emitConnectionError" => self.emit_connection_error(get_param!(msg)),
            "ircUserNickChanged" => self.irc_user_nick_changed(get_param!(msg), get_param!(msg)),
            "removeCap" => self.remove_cap(get_param!(msg)),
            "removeSupport" => self.remove_support(get_param!(msg)),
            "setAutoIdentifyPassword" => self.network_info.set_auto_identify_password(get_param!(msg)),
            "setAutoIdentifyService" => self.network_info.set_auto_identify_service(get_param!(msg)),
            "setAutoReconnectInterval" => self.network_info.set_auto_reconnect_interval(get_param!(msg)),
            "setAutoReconnectRetries" => self.network_info.set_auto_reconnect_retries(get_param!(msg)),
            "setCodecForDecoding" => self.network_info.set_codec_for_decoding(get_param!(msg)),
            "setCodecForEncoding" => self.network_info.set_codec_for_encoding(get_param!(msg)),
            "setCodecForServer" => self.network_info.set_codec_for_server(get_param!(msg)),
            "setConnected" => self.set_connected(get_param!(msg)),
            "setConnectionState" => self.set_connection_state(get_param!(msg)),
            "setCurrentServer" => self.set_current_server(get_param!(msg)),
            "setIdentity" => self.network_info.set_identity_id(get_param!(msg)),
            "setLatency" => self.set_latency(get_param!(msg)),
            "setMessageRateBurstSize" => self.network_info.set_msg_rate_burst_size(get_param!(msg)),
            "setMessageRateDelay" => self.network_info.set_msg_rate_message_delay(get_param!(msg)),
            "setMyNick" => self.set_my_nick(get_param!(msg)),
            "setNetworkName" => self.network_info.set_network_name(get_param!(msg)),
            "setNetworkInfo" => self.set_network_info(NetworkInfo::from_network_map(
                &mut VariantMap::try_from(msg.params.remove(0)).unwrap(),
            )),
            "setPerform" => self.network_info.set_perform(get_param!(msg)),
            "setRejoinChannels" => self.network_info.set_rejoin_channels(get_param!(msg)),
            "setSaslAccount" => self.network_info.set_sasl_account(get_param!(msg)),
            "setSaslPassword" => self.network_info.set_sasl_password(get_param!(msg)),
            // "setServerList" => self.network_info.set_server_list(get_param!(msg)),
            "setActualServerList" => self.network_info.set_server_list({
                match msg.params.remove(0) {
                    Variant::VariantList(mut variants) => {
                        Vec::<NetworkServer>::from_network_map(&mut variants)
                    }
                    _ => {
                        error!("{}", ProtocolError::WrongVariant);
                        // TODO FIXME
                        Vec::new()
                    }
                }
            }),
            "setUnlimitedMessageRate" => self.network_info.set_unlimited_message_rate(get_param!(msg)),
            "setUnlimitedReconnectRetries" => {
                self.network_info.set_unlimited_reconnect_retries(get_param!(msg))
            }
            "setUseAutoIdentify" => self.network_info.set_use_auto_identify(get_param!(msg)),
            "setUseAutoReconnect" => self.network_info.set_use_auto_reconnect(get_param!(msg)),
            "setUseCustomMessageRate" => self.network_info.set_use_custom_message_rate(get_param!(msg)),
            "setUseRandomServer" => self.network_info.set_use_random_server(get_param!(msg)),
            "setUseSasl" => self.network_info.set_use_sasl(get_param!(msg)),
            _ => (),
        }
    }
}

#[cfg(feature = "server")]
impl crate::message::StatefulSyncableServer for Network {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "requestConnect" => self.connect(),
            "requestDisconnect" => self.disconnect(),
            "requestSetNetworkInfo" => self.set_network_info(NetworkInfo::from_network_map(
                &mut VariantMap::try_from(msg.params.remove(0)).unwrap(),
            )),
            _ => (),
        }
    }
}

impl crate::message::signalproxy::NetworkList for Network {
    fn to_network_list(&self) -> VariantList {
        let mut res = VariantList::new();

        res.push(Variant::ByteArray(s!("myNick")));
        res.push(Variant::String(self.my_nick.clone()));
        res.push(Variant::ByteArray(s!("latency")));
        res.push(Variant::i32(self.latency));
        res.push(Variant::ByteArray(s!("currentServer")));
        res.push(Variant::String(self.current_server.clone()));
        res.push(Variant::ByteArray(s!("isConnected")));
        res.push(Variant::bool(self.is_connected));
        res.push(Variant::ByteArray(s!("connectionState")));
        res.push(Variant::i32(self.connection_state as i32));

        res.push(Variant::ByteArray(s!("Supports")));
        res.push(Variant::VariantMap(
            self.supports
                .iter()
                .map(|(k, v)| (k.clone(), Variant::String(v.clone())))
                .collect(),
        ));

        res.push(Variant::ByteArray(s!("Caps")));
        res.push(Variant::VariantMap(
            self.caps
                .iter()
                .map(|(k, v)| (k.clone(), Variant::String(v.clone())))
                .collect(),
        ));

        res.push(Variant::ByteArray(s!("CapsEnabled")));
        res.push(Variant::VariantList(
            self.caps_enabled
                .iter()
                .map(|v| Variant::String(v.clone()))
                .collect(),
        ));

        {
            let mut map = VariantMap::new();

            map.insert(
                s!("Users"),
                Variant::VariantMap(self.irc_users.iter().fold(HashMap::new(), |mut res, (_, v)| {
                    res.extend(v.to_network_map());

                    res
                })),
            );

            let channels = self.irc_channels.iter().fold(HashMap::new(), |mut res, (_, v)| {
                res.extend(v.to_network_map());

                res
            });

            map.insert(s!("Channels"), Variant::VariantMap(channels));

            res.push(Variant::ByteArray(s!("IrcUsersAndChannels")));
            res.push(Variant::VariantMap(map));
        }

        res.extend(self.network_info.to_network_list());

        res
    }

    fn from_network_list(input: &mut VariantList) -> Self {
        let mut i = input.iter().cycle();

        let users_and_channels: VariantMap = {
            i.position(|x| *x == Variant::ByteArray(String::from("IrcUsersAndChannels")))
                .unwrap();

            i.next().unwrap().try_into().unwrap()
        };

        log::trace!("users and channels: {:#?}", users_and_channels);

        let mut network = Self {
            my_nick: {
                i.position(|x| *x == Variant::ByteArray(String::from("myNick")))
                    .unwrap();

                i.next().unwrap().try_into().unwrap()
            },
            latency: {
                i.position(|x| *x == Variant::ByteArray(String::from("latency")))
                    .unwrap();

                i.next().unwrap().try_into().unwrap()
            },
            current_server: {
                i.position(|x| *x == Variant::ByteArray(String::from("currentServer")))
                    .unwrap();

                i.next().unwrap().try_into().unwrap()
            },
            is_connected: {
                i.position(|x| *x == Variant::ByteArray(String::from("isConnected")))
                    .unwrap();

                i.next().unwrap().try_into().unwrap()
            },
            connection_state: ConnectionState::from_i32({
                i.position(|x| *x == Variant::ByteArray(String::from("connectionState")))
                    .unwrap();

                i.next().unwrap().try_into().unwrap()
            })
            .unwrap(),
            prefixes: Vec::new(),
            prefix_modes: Vec::new(),
            channel_modes: HashMap::with_capacity(4),
            irc_users: {
                match users_and_channels.get("Users") {
                    Some(users) => {
                        let users: Vec<IrcUser> = Vec::<IrcUser>::from_network_map(
                            &mut users.try_into().expect("failed to convert Users"),
                        );

                        users.into_iter().map(|user| (user.nick.clone(), user)).collect()
                    }
                    None => HashMap::new(),
                }
            },
            irc_channels: {
                match users_and_channels.get("Channels") {
                    Some(channels) => {
                        let channels: Vec<IrcChannel> =
                            Vec::<IrcChannel>::from_network_map(&mut channels.try_into().unwrap());
                        channels
                            .into_iter()
                            .map(|channel| (channel.name.clone(), channel))
                            .collect()
                    }
                    None => HashMap::new(),
                }
            },
            supports: {
                i.position(|x| *x == Variant::ByteArray(String::from("Supports")))
                    .unwrap();

                let var: VariantMap = i.next().unwrap().try_into().unwrap();

                var.into_iter().map(|(k, v)| (k, v.try_into().unwrap())).collect()
            },
            caps: {
                i.position(|x| *x == Variant::ByteArray(String::from("Caps")))
                    .unwrap();

                let var: VariantMap = i.next().unwrap().try_into().unwrap();

                var.into_iter().map(|(k, v)| (k, v.try_into().unwrap())).collect()
            },
            caps_enabled: {
                i.position(|x| *x == Variant::ByteArray(String::from("CapsEnabled")))
                    .unwrap();

                let var: VariantList = i.next().unwrap().try_into().unwrap();

                var.into_iter().map(|v| v.try_into().unwrap()).collect()
            },
            network_info: NetworkInfo::from_network_list(input),
        };

        network.determine_channel_mode_types();
        network.determine_prefixes();

        return network;
    }
}

impl crate::message::signalproxy::NetworkMap for Network {
    type Item = VariantMap;

    fn to_network_map(&self) -> Self::Item {
        let mut res = VariantMap::new();

        res.insert("myNick".to_owned(), Variant::String(self.my_nick.clone()));
        res.insert("latency".to_owned(), Variant::i32(self.latency));
        res.insert(
            "currentServer".to_owned(),
            Variant::String(self.current_server.clone()),
        );
        res.insert("isConnected".to_owned(), Variant::bool(self.is_connected));
        res.insert(
            "connectionState".to_owned(),
            Variant::i32(self.connection_state as i32),
        );

        res.insert(
            "Supports".to_owned(),
            Variant::VariantMap(
                self.supports
                    .iter()
                    .map(|(k, v)| (k.clone(), Variant::String(v.clone())))
                    .collect(),
            ),
        );

        res.insert(
            "Caps".to_owned(),
            Variant::VariantMap(
                self.caps
                    .iter()
                    .map(|(k, v)| (k.clone(), Variant::String(v.clone())))
                    .collect(),
            ),
        );

        res.insert(
            "CapsEnabled".to_owned(),
            Variant::VariantList(
                self.caps_enabled
                    .iter()
                    .map(|v| Variant::String(v.clone()))
                    .collect(),
            ),
        );

        res.insert(s!("IrcUsersAndChannels"), {
            let mut map = VariantMap::new();

            map.insert(
                s!("Users"),
                Variant::VariantMap(self.irc_users.iter().fold(HashMap::new(), |mut res, (_, v)| {
                    res.extend(v.to_network_map());

                    res
                })),
            );

            let channels = self.irc_channels.iter().fold(HashMap::new(), |mut res, (_, v)| {
                res.extend(v.to_network_map());

                res
            });

            map.insert(s!("Channels"), Variant::VariantMap(channels));

            Variant::VariantMap(map)
        });

        res.extend(self.network_info.to_network_map());

        return res;
    }

    fn from_network_map(input: &mut Self::Item) -> Self {
        let users_and_channels: VariantMap =
            { input.get("IrcUsersAndChannels").unwrap().try_into().unwrap() };

        return Self {
            my_nick: input.get("myNick").unwrap().into(),
            latency: input.get("latency").unwrap().try_into().unwrap(),
            current_server: input.get("currentServer").unwrap().into(),
            is_connected: input.get("isConnected").unwrap().try_into().unwrap(),
            connection_state: ConnectionState::from_i32(
                input.get("connectionState").unwrap().try_into().unwrap(),
            )
            .unwrap(),
            prefixes: Vec::new(),
            prefix_modes: Vec::new(),
            channel_modes: HashMap::with_capacity(4),
            irc_users: {
                match users_and_channels.get("Users") {
                    Some(users) => {
                        let users: Vec<IrcUser> = Vec::<IrcUser>::from_network_map(
                            &mut users.try_into().expect("failed to convert Users"),
                        );

                        users.into_iter().map(|user| (user.nick.clone(), user)).collect()
                    }
                    None => HashMap::new(),
                }
            },
            irc_channels: {
                match users_and_channels.get("Channels") {
                    Some(channels) => {
                        let channels: Vec<IrcChannel> =
                            Vec::<IrcChannel>::from_network_map(&mut channels.try_into().unwrap());
                        channels
                            .into_iter()
                            .map(|channel| (channel.name.clone(), channel))
                            .collect()
                    }
                    None => HashMap::new(),
                }
            },
            supports: VariantMap::try_from(input.get("Supports").unwrap())
                .unwrap()
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            caps: VariantMap::try_from(input.get("Caps").unwrap())
                .unwrap()
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            caps_enabled: VariantList::try_from(input.get("CapsEnabled").unwrap())
                .unwrap()
                .into_iter()
                .map(|v| v.into())
                .collect(),
            network_info: NetworkInfo::from_network_map(input),
        };
    }
}

#[derive(Debug, Clone, PartialEq, NetworkMap)]
pub struct NetworkServer {
    #[network(rename = "Host")]
    pub host: String,
    #[network(rename = "Port")]
    pub port: u32,
    #[network(rename = "Password")]
    pub password: String,
    #[network(rename = "UseSSL")]
    pub use_ssl: bool,
    #[network(rename = "sslVerify")]
    pub ssl_verify: bool,
    #[network(rename = "sslVersion")]
    pub ssl_version: i32,
    #[network(rename = "UseProxy")]
    pub use_proxy: bool,
    #[network(rename = "ProxyType")]
    pub proxy_type: i32,
    #[network(rename = "ProxyHost")]
    pub proxy_host: String,
    #[network(rename = "ProxyPort")]
    pub proxy_port: u32,
    #[network(rename = "ProxyUser")]
    pub proxy_user: String,
    #[network(rename = "ProxyPass")]
    pub proxy_pass: String,
}

// TODO this is not correct usage, it's technically not really network repr were converting from
// but just the conversion of VariantList -> Self directly
// we have this problem since now we have generic VariantList impls
// for all the variants and this type is now also directly a variant
impl NetworkList for Vec<NetworkServer> {
    fn to_network_list(&self) -> super::VariantList {
        self.iter().map(|b| Variant::NetworkServer(b.clone())).collect()
    }

    fn from_network_list(input: &mut super::VariantList) -> Self {
        input.iter().map(|b| match_variant!(b, Variant::NetworkServer)).collect()
    }
}

impl UserType for NetworkServer {
    const NAME: &str = "Network::Server";
}

impl Serialize for NetworkServer {
    fn serialize(&self) -> Result<Vec<u8>, crate::ProtocolError> {
        self.to_network_map().serialize()
    }
}

impl Deserialize for NetworkServer {
    fn parse(b: &[u8]) -> Result<(usize, Self), crate::ProtocolError>
    where
        Self: std::marker::Sized,
    {
        let (vlen, mut value) = VariantMap::parse(b)?;
        return Ok((vlen, Self::from_network_map(&mut value)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn networkserver_get_network() -> VariantMap {
        map! {
            s!("ProxyHost") => Variant::String(
                s!("localhost"),
            ),
            s!("sslVerify") => Variant::bool(
                true,
            ),
            s!("UseSSL") => Variant::bool(
                true,
            ),
            s!("Port") => Variant::u32(
                6697,
            ),
            s!("Password") => Variant::String(
                s!(""),
            ),
            s!("ProxyType") => Variant::i32(
                1,
            ),
            s!("sslVersion") => Variant::i32(
                0,
            ),
            s!("ProxyUser") => Variant::String(
                s!(""),
            ),
            s!("ProxyPass") => Variant::String(
                s!(""),
            ),
            s!("Host") => Variant::String(
                s!("irc.snoonet.org"),
            ),
            s!("ProxyPort") => Variant::u32(
                8080,
            ),
            s!("UseProxy") => Variant::bool(
                false,
            ),
        }
    }
    fn networkserver_get_runtime() -> NetworkServer {
        NetworkServer {
            host: s!("irc.snoonet.org"),
            port: 6697,
            password: s!(""),
            use_ssl: true,
            ssl_verify: true,
            ssl_version: 0,
            use_proxy: false,
            proxy_type: 1,
            proxy_host: s!("localhost"),
            proxy_port: 8080,
            proxy_user: s!(""),
            proxy_pass: s!(""),
        }
    }

    #[test]
    fn network_server_to_network() {
        assert_eq!(
            networkserver_get_runtime().to_network_map(),
            networkserver_get_network()
        )
    }

    #[test]
    fn network_determine_channel_modes() {
        let mut network = Network::default();

        network.supports.insert(
            s!("CHANMODES"),
            s!("IXZbegw,k,FHJLWdfjlx,ABCDKMNOPQRSTcimnprstuz"),
        );

        network.determine_channel_mode_types();

        assert_eq!(
            network.channel_modes.get(&ChannelModeType::AChanmode).unwrap(),
            "IXZbegw"
        );
        assert_eq!(
            network.channel_modes.get(&ChannelModeType::BChanmode).unwrap(),
            "k"
        );
        assert_eq!(
            network.channel_modes.get(&ChannelModeType::CChanmode).unwrap(),
            "FHJLWdfjlx"
        );
        assert_eq!(
            network.channel_modes.get(&ChannelModeType::DChanmode).unwrap(),
            "ABCDKMNOPQRSTcimnprstuz"
        );
    }

    #[test]
    fn network_get_channel_mode_type() {
        let mut network = Network::default();

        network.supports.insert(
            s!("CHANMODES"),
            s!("IXZbegw,k,FHJLWdfjlx,ABCDKMNOPQRSTcimnprstuz"),
        );
        network.determine_channel_mode_types();

        assert_eq!(network.get_channel_mode_type('b'), ChannelModeType::AChanmode);
        assert_eq!(network.get_channel_mode_type('k'), ChannelModeType::BChanmode);
        assert_eq!(network.get_channel_mode_type('W'), ChannelModeType::CChanmode);
        assert_eq!(network.get_channel_mode_type('D'), ChannelModeType::DChanmode);
        assert_eq!(network.get_channel_mode_type('E'), ChannelModeType::NotAChanmode);
    }

    #[test]
    fn network_determine_prefixes() {
        let mut network = Network::default();
        network.determine_prefixes();

        assert_eq!(network.prefixes, vec!['~', '&', '@', '%', '+']);
        assert_eq!(network.prefix_modes, vec!['q', 'a', 'o', 'h', 'v']);

        network.supports.insert(s!("PREFIX"), s!("(Yohv)!@%+"));

        network.determine_prefixes();

        assert_eq!(network.prefixes, vec!['!', '@', '%', '+']);
        assert_eq!(network.prefix_modes, vec!['Y', 'o', 'h', 'v']);
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
pub enum ConnectionState {
    Disconnected = 0x00,
    Connecting = 0x01,
    Initializing = 0x02,
    Initialized = 0x03,
    Reconnecting = 0x04,
    Disconnecting = 0x05,
}

impl Default for ConnectionState {
    fn default() -> Self {
        Self::Disconnected
    }
}

impl Into<Variant> for ConnectionState {
    fn into(self) -> Variant {
        Variant::i32(self.to_i32().unwrap())
    }
}

impl TryFrom<Variant> for ConnectionState {
    type Error = ProtocolError;

    fn try_from(value: Variant) -> Result<Self, Self::Error> {
        match value {
            Variant::i32(n) => Ok(ConnectionState::from_i32(n).unwrap()),
            _ => Err(ProtocolError::WrongVariant),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
pub enum ChannelModeType {
    NotAChanmode = 0x00,
    AChanmode = 0x01,
    BChanmode = 0x02,
    CChanmode = 0x04,
    DChanmode = 0x08,
}
