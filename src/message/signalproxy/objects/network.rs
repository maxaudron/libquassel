use std::collections::HashMap;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use libquassel_derive::{NetworkList, NetworkMap};

use crate::message::signalproxy::translation::NetworkMap;
use crate::primitive::{Variant, VariantList, VariantMap};

use super::{ircchannel::IrcChannel, ircuser::IrcUser, networkinfo::NetworkInfo};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Network {
    pub my_nick: String,
    pub latency: i32,
    pub current_server: String,
    pub is_connected: bool,
    pub connection_state: ConnectionState,
    pub prefixes: Vec<char>,
    pub prefix_modes: Vec<char>,
    pub channel_modes: HashMap<ChannelModeType, String>,
    pub irc_users: HashMap<String, IrcUser>,
    pub irc_channels: HashMap<String, IrcChannel>,
    pub supports: HashMap<String, String>,
    pub caps: HashMap<String, String>,
    pub caps_enabled: Vec<String>,
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
        res.push(Variant::i32(self.connection_state.clone() as i32));

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

#[derive(Debug, Clone, PartialEq, NetworkList)]
pub struct NetworkConfig {
    #[network(rename = "pingTimeoutEnabled")]
    ping_timeout_enabled: bool,
    #[network(rename = "pingInterval")]
    ping_interval: i32,
    #[network(rename = "maxPingCount")]
    max_ping_count: i32,
    #[network(rename = "autoWhoEnabled")]
    auto_who_enabled: bool,
    #[network(rename = "autoWhoInterval")]
    auto_who_interval: i32,
    #[network(rename = "autoWhoNickLimit")]
    auto_who_nick_limit: i32,
    #[network(rename = "autoWhoDelay")]
    auto_who_delay: i32,
    #[network(rename = "standardCtcp")]
    standard_ctcp: bool,
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
#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
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
