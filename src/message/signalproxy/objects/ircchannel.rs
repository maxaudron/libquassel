use std::collections::HashMap;

#[cfg(feature = "server")]
use libquassel_derive::sync;
use libquassel_derive::{NetworkList, NetworkMap, Setters};
use log::{error, warn};

use crate::message::{signalproxy::translation::NetworkMap, Class, Syncable};
use crate::primitive::{StringList, VariantMap};
use crate::serialize::{Deserialize, Serialize, UserType};

use super::{ChanModes, ChannelModeType};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Setters, NetworkList, NetworkMap)]
#[network(repr = "maplist")]
pub struct IrcChannel {
    #[setter(skip)]
    #[network(rename = "ChanModes", variant = "VariantMap", network = "map")]
    pub chan_modes: ChanModes,

    // pub channel_modes: HashMap<char, ChannelMode>,
    #[setter(skip)]
    #[network(rename = "UserModes", variant = "VariantMap", network = "map")]
    pub user_modes: HashMap<String, String>,
    #[setter(skip)]
    pub name: String,

    pub topic: String,
    pub password: String,
    pub encrypted: bool,
}

impl UserType for IrcChannel {
    const NAME: &str = "IrcChannel";
}

impl Serialize for IrcChannel {
    fn serialize(&self) -> Result<Vec<u8>, crate::ProtocolError> {
        self.to_network_map().serialize()
    }
}

impl Deserialize for IrcChannel {
    fn parse(b: &[u8]) -> Result<(usize, Self), crate::ProtocolError>
    where
        Self: std::marker::Sized,
    {
        let (vlen, mut value) = VariantMap::parse(b)?;
        return Ok((vlen, Self::from_network_map(&mut value)));
    }
}

// TODO keep user modes sorted
impl IrcChannel {
    pub fn add_channel_mode(&mut self, mode_type: ChannelModeType, mode: char, value: String) {
        match mode_type {
            ChannelModeType::NotAChanmode => (),
            ChannelModeType::AChanmode => {
                self.chan_modes.channel_modes_a.insert(mode, vec![value]);
            }
            ChannelModeType::BChanmode => {
                self.chan_modes.channel_modes_b.insert(mode, value);
            }
            ChannelModeType::CChanmode => {
                self.chan_modes.channel_modes_c.insert(mode, value);
            }
            ChannelModeType::DChanmode => {
                if !self.chan_modes.channel_modes_d.contains(mode) {
                    self.chan_modes.channel_modes_d.push(mode);
                };
            }
        };
    }
    pub fn remove_channel_mode(&mut self, mode_type: ChannelModeType, mode: char, _value: String) {
        match mode_type {
            ChannelModeType::NotAChanmode => (),
            ChannelModeType::AChanmode => {
                self.chan_modes.channel_modes_a.remove(&mode);
            }
            ChannelModeType::BChanmode => {
                self.chan_modes.channel_modes_b.remove(&mode);
            }
            ChannelModeType::CChanmode => {
                self.chan_modes.channel_modes_c.remove(&mode);
            }
            ChannelModeType::DChanmode => {
                if self.chan_modes.channel_modes_d.contains(mode) {
                    self.chan_modes.channel_modes_d = self
                        .chan_modes
                        .channel_modes_d
                        .chars()
                        .filter(|c| *c != mode)
                        .collect();
                };
            }
        }
    }

    // TODO add user mode validation
    /// Add one or more mode flags to a user
    pub fn add_user_mode(&mut self, nick: String, mode: String) {
        if let Some(user_modes) = self.user_modes.get_mut(&nick) {
            mode.chars().for_each(|c| {
                if !user_modes.contains(c) {
                    user_modes.push(c);
                }
            });
        } else {
            self.user_modes.insert(nick.clone(), mode.clone());
        };

        // We need to iterate over all the chars and send a sync for each one
        // to stay compatible with quassels current behaviour
        // TODO this might actually be dumb can IRC even into mutiple modes at once?
        #[cfg(feature = "server")]
        if let Some(user_modes) = self.user_modes.get(&nick) {
            mode.chars().for_each(|c| {
                if !user_modes.contains(c) {
                    sync!("addUserMode", [nick.clone(), c.to_string()]);
                }
            });
        };
    }

    /// Remove one or more mode flags from a user
    pub fn remove_user_mode(&mut self, nick: String, mode: String) {
        if let Some(user_modes) = self.user_modes.get_mut(&nick) {
            mode.chars().for_each(|c| {
                *user_modes = user_modes.replace(c, "");
            });
        }

        #[cfg(feature = "server")]
        sync!("removeUserMode", [nick, mode]);
    }

    pub fn join_irc_users(&mut self, nicks: StringList, modes: StringList) {
        if nicks.len() != modes.len() {
            error!("number of nicks does not match number of modes");
        }

        #[cfg(feature = "server")]
        sync!("joinIrcUsers", [nicks.clone(), modes.clone()]);

        nicks
            .into_iter()
            .zip(modes)
            .for_each(|(nick, mode)| self.add_user_mode(nick, mode));
    }

    pub fn part(&mut self, nick: String) {
        match self.user_modes.remove(&nick) {
            Some(_) => (),
            None => warn!("tried to remove a user that is not joined to the channel"),
        }

        if self.user_modes.len() == 0
        /* nick.is_me() */
        {
            // TODO Clean up channel and delete
        }
    }

    pub fn set_user_modes(&mut self, nick: String, modes: String) {
        #[cfg(feature = "server")]
        sync!("setUserModes", [nick.clone(), modes.clone()]);

        *self.user_modes.entry(nick).or_default() = modes;
    }
}

#[cfg(feature = "client")]
impl crate::message::StatefulSyncableClient for IrcChannel {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            // "addChannelMode" => {
            //     let mode: String = get_param!(msg);
            //     self.add_channel_mode(mode.chars().next().unwrap(), get_param!(msg));
            // }
            // "removeChannelMode" => {
            //     let mode: String = get_param!(msg);
            //     self.remove_channel_mode(mode.chars().next().unwrap(), get_param!(msg));
            // }
            "addUserMode" => self.add_user_mode(get_param!(msg), get_param!(msg)),
            "removeUserMode" => self.remove_user_mode(get_param!(msg), get_param!(msg)),
            "joinIrcUsers" => self.join_irc_users(get_param!(msg), get_param!(msg)),
            "part" => self.part(get_param!(msg)),
            "setEncrypted" => self.set_encrypted(get_param!(msg)),
            "setPassword" => self.set_password(get_param!(msg)),
            "setTopic" => self.set_topic(get_param!(msg)),
            "setUserModes" => self.set_user_modes(get_param!(msg), get_param!(msg)),
            _ => (),
        }
    }

    /// Not Implemented for this type
    fn request_update(&mut self)
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

#[cfg(feature = "server")]
impl crate::message::StatefulSyncableServer for IrcChannel {
    /// Not Implemented for this type
    fn request_update(&mut self, _param: <IrcChannel as crate::message::NetworkMap>::Item)
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl Syncable for IrcChannel {
    const CLASS: Class = Class::IrcChannel;
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::message::NetworkMap;
    use crate::primitive::{Variant, VariantMap};

    fn get_network() -> VariantMap {
        map! {
            s!("encrypted") =>
                    Variant::VariantList(vec![Variant::bool(
                        false,
                    )]),
            s!("topic") =>
                    Variant::VariantList(vec![Variant::String(
                        s!(""),
                    )]),
            s!("password") =>
                    Variant::VariantList(vec![Variant::String(
                        s!(""),
                    )]),
            s!("ChanModes") => Variant::VariantList(vec![Variant::VariantMap(map!
                        {
                            s!("B") => Variant::VariantMap(map!
                                {},
                            ),
                            s!("D") => Variant::String(
                                s!("tCnT"),
                            ),
                            s!("C") => Variant::VariantMap(map!
                                {
                                    s!("j") => Variant::String(
                                        s!("5:1"),
                                    ),
                                    s!("x") => Variant::String(
                                        s!("10:5"),
                                    ),
                                    s!("f") => Variant::String(
                                        s!("30:5"),
                                    ),
                                    s!("F") => Variant::String(
                                        s!("5:60"),
                                    ),
                                },
                            ),
                            s!("A") => Variant::VariantMap(map! {
                                s!("b") => Variant::StringList(vec![s!("*!*@test"), s!("*!*@test2")]),
                            }),
                        },
                    )]),
            s!("UserModes") =>
                    Variant::VariantList(vec![Variant::VariantMap(map!
                        {
                            s!("audron") => Variant::String(
                                s!("o"),
                            ),
                            s!("audron_") => Variant::String(
                                s!(""),
                            ),
                        },
                    )]),
            s!("name") =>
                    Variant::VariantList(vec![Variant::String(
                        s!("#audron-test"),
                    )]),
        }
    }
    fn get_runtime() -> IrcChannel {
        IrcChannel {
            chan_modes: ChanModes {
                channel_modes_a: map! { 'b' => vec![s!("*!*@test"), s!("*!*@test2")] },
                channel_modes_b: map! {},
                channel_modes_c: map! { 'j' => s!("5:1"), 'x' => s!("10:5"), 'f' => s!("30:5"), 'F' => s!("5:60") },
                channel_modes_d: s!("tCnT"),
            },
            user_modes: map! { s!("audron") => s!("o"), s!("audron_") => s!("") },
            name: s!("#audron-test"),
            topic: s!(""),
            password: s!(""),
            encrypted: false,
        }
    }

    #[test]
    fn ircchannel_to_network() {
        assert_eq!(get_runtime().to_network_map(), get_network())
    }

    #[test]
    fn ircchannel_from_network() {
        assert_eq!(IrcChannel::from_network_map(&mut get_network()), get_runtime())
    }

    #[test]
    fn add_user_mode() {
        let mut base = get_runtime();
        let mut res = get_runtime();
        res.user_modes = map! { s!("audron") => s!("oh"), s!("audron_") => s!("") };

        base.add_user_mode(s!("audron"), s!("h"));
        assert_eq!(res, base);
        base.add_user_mode(s!("audron"), s!("o"));
        assert_eq!(res, base);

        res.user_modes = map! { s!("audron") => s!("oh"), s!("audron_") => s!(""), s!("test") => s!("h") };
        base.add_user_mode(s!("test"), s!("h"));
        assert_eq!(res, base);
    }
}
