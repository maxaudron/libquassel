use crate::{
    message::{Class, NetworkMap, Syncable},
    primitive::{DateTime, StringList, VariantMap},
    serialize::{Deserialize, Serialize, UserType},
};

use itertools::Itertools;
#[cfg(feature = "server")]
use libquassel_derive::sync;
use libquassel_derive::{NetworkList, NetworkMap, Setters};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, NetworkList, NetworkMap, Setters)]
#[network(repr = "maplist")]
pub struct IrcUser {
    pub user: String,
    pub host: String,
    pub nick: String,
    #[network(rename = "realName")]
    pub real_name: String,
    pub account: String,
    pub away: bool,
    #[network(rename = "awayMessage")]
    pub away_message: String,
    #[network(rename = "idleTime")]
    pub idle_time: DateTime,
    #[network(rename = "loginTime")]
    pub login_time: DateTime,
    pub server: String,
    #[network(rename = "ircOperator")]
    pub irc_operator: String,
    // #[quassel(name = "lastAwayMessage")]
    // pub last_away_message: i32,
    #[network(rename = "lastAwayMessageTime")]
    pub last_away_message_time: DateTime,
    #[network(rename = "whoisServiceReply")]
    pub whois_service_reply: String,
    #[network(rename = "suserHost")]
    pub suser_host: String,
    pub encrypted: bool,
    pub channels: StringList,
    #[network(rename = "userModes")]
    pub user_modes: String,
}

impl UserType for IrcUser {
    const NAME: &str = "IrcUser";
}

impl Serialize for IrcUser {
    fn serialize(&self) -> Result<Vec<u8>, crate::ProtocolError> {
        self.to_network_map().serialize()
    }
}

impl Deserialize for IrcUser {
    fn parse(b: &[u8]) -> Result<(usize, Self), crate::ProtocolError>
    where
        Self: std::marker::Sized,
    {
        let (vlen, mut value) = VariantMap::parse(b)?;
        return Ok((vlen, Self::from_network_map(&mut value)));
    }
}

impl IrcUser {
    pub fn add_user_modes(&mut self, modes: String) {
        for mode in modes.chars() {
            if !self.user_modes.contains(mode) {
                self.user_modes.push(mode);
            }
        }

        #[cfg(feature = "server")]
        sync!("addUserModes", [modes]);
    }

    pub fn remove_user_modes(&mut self, modes: String) {
        for mode in modes.chars() {
            if self.user_modes.contains(mode) {
                self.user_modes = self.user_modes.chars().filter(|c| *c != mode).collect();
            }
        }

        #[cfg(feature = "server")]
        sync!("removeUserModes", [modes]);
    }

    pub fn update_hostmask(&mut self, _mask: String) {}

    pub fn join_channel(&mut self, channel: String) {
        if !self.channels.contains(&channel) {
            self.channels.push(channel.clone())
        }

        #[cfg(feature = "server")]
        sync!("partChannel", [channel]);
    }

    pub fn part_channel(&mut self, channel: String) {
        if let Some((i, _)) = self.channels.iter().find_position(|c| **c == channel) {
            self.channels.remove(i);
        }

        #[cfg(feature = "server")]
        sync!("partChannel", [channel]);
    }

    pub fn quit(&mut self) {}
}

#[cfg(feature = "client")]
impl crate::message::StatefulSyncableClient for IrcUser {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "addUserModes" => self.add_user_modes(get_param!(msg)),
            "joinChannel" => self.join_channel(get_param!(msg)),
            "partChannel" => self.part_channel(get_param!(msg)),
            "quit" => self.quit(),
            "removeUserModes" => self.remove_user_modes(get_param!(msg)),
            "setAccount" => self.set_account(get_param!(msg)),
            "setAway" => self.set_away(get_param!(msg)),
            "setAwayMessage" => self.set_away_message(get_param!(msg)),
            "setEncrypted" => self.set_encrypted(get_param!(msg)),
            "setHost" => self.set_host(get_param!(msg)),
            "setIdleTime" => self.set_idle_time(get_param!(msg)),
            "setIrcOperator" => self.set_irc_operator(get_param!(msg)),
            // TODO
            // "setLastAwayMessage" => self.,
            "setLastAwayMessageTime" => self.set_last_away_message_time(get_param!(msg)),
            "setLoginTime" => self.set_login_time(get_param!(msg)),
            "setNick" => self.set_nick(get_param!(msg)),
            "setRealName" => self.set_real_name(get_param!(msg)),
            "setServer" => self.set_server(get_param!(msg)),
            "setSuserHost" => self.set_suser_host(get_param!(msg)),
            "setUser" => self.set_user(get_param!(msg)),
            "setUserModes" => self.set_user_modes(get_param!(msg)),
            "setWhoisServiceReply" => self.set_whois_service_reply(get_param!(msg)),
            "updateHostmask" => self.update_hostmask(get_param!(msg)),
            _ => unimplemented!(),
        }
    }
}

#[cfg(feature = "server")]
impl crate::message::StatefulSyncableServer for IrcUser {}

impl Syncable for IrcUser {
    const CLASS: Class = Class::IrcUser;

    fn send_sync(&self, function: &str, params: crate::primitive::VariantList) {
        crate::message::signalproxy::SYNC_PROXY
            .get()
            .unwrap()
            .sync(Self::CLASS, None, function, params);
    }
}

#[cfg(test)]
mod tests {
    use crate::message::signalproxy::NetworkMap;
    use crate::primitive::{Variant, VariantMap};
    use time::OffsetDateTime;

    use super::*;

    fn get_runtime() -> IrcUser {
        IrcUser {
            user: s!("NickServ"),
            host: s!("services"),
            nick: s!("NickServ"),
            real_name: s!(""),
            account: s!(""),
            away: false,
            away_message: s!(""),
            idle_time: OffsetDateTime::UNIX_EPOCH,
            login_time: OffsetDateTime::UNIX_EPOCH,
            server: s!(""),
            irc_operator: s!(""),
            // last_away_message: 0,
            last_away_message_time: OffsetDateTime::UNIX_EPOCH,
            whois_service_reply: s!(""),
            suser_host: s!(""),
            encrypted: false,
            channels: StringList::new(),
            user_modes: s!(""),
        }
    }

    fn get_network() -> VariantMap {
        map! {
            s!("suserHost") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("lastAwayMessageTime") => Variant::VariantList(vec!
                [
                    Variant::DateTime(
                        OffsetDateTime::UNIX_EPOCH,
                    ),
                ],
            ),
            s!("away") => Variant::VariantList(vec!
                [
                    Variant::bool(
                        false,
                    ),
                ],
            ),
            s!("ircOperator") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("account") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("loginTime") => Variant::VariantList(vec!
                [
                    Variant::DateTime(
                        OffsetDateTime::UNIX_EPOCH
                    ),
                ],
            ),
            s!("userModes") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("host") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!("services"),
                    ),
                ],
            ),
            s!("whoisServiceReply") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("channels") => Variant::VariantList(vec!
                [
                    Variant::StringList(vec!
                        [],
                    ),
                ],
            ),
            s!("realName") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("nick") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!("NickServ"),
                    ),
                ],
            ),
            s!("idleTime") => Variant::VariantList(vec!
                [
                    Variant::DateTime(
                        OffsetDateTime::UNIX_EPOCH
                    ),
                ],
            ),
            s!("encrypted") => Variant::VariantList(vec!
                [
                    Variant::bool(
                        false,
                    ),
                ],
            ),
            s!("awayMessage") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("user") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!("NickServ"),
                    ),
                ],
            ),
            s!("server") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
        }
    }

    #[test]
    fn ircuser_to_network() {
        assert_eq!(get_runtime().to_network_map(), get_network())
    }

    #[test]
    fn ircuser_from_network() {
        assert_eq!(IrcUser::from_network_map(&mut get_network()), get_runtime())
    }

    #[test]
    fn vec_ircuser_to_network() {
        assert_eq!(get_runtime().to_network_map(), get_network())
    }
}
