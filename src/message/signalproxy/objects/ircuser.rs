use crate::{
    message::{Class, Syncable},
    primitive::{DateTime, StringList},
};

use itertools::Itertools;
#[cfg(feature = "server")]
use libquassel_derive::sync;
use libquassel_derive::{NetworkMap, Setters};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, NetworkMap, Setters)]
#[network(repr = "maplist")]
pub struct IrcUser {
    pub user: String,
    pub host: String,
    pub nick: String,
    #[quassel(name = "realName")]
    pub real_name: String,
    pub account: String,
    pub away: bool,
    #[quassel(name = "awayMessage")]
    pub away_message: String,
    #[quassel(name = "idleTime")]
    pub idle_time: DateTime,
    #[quassel(name = "loginTime")]
    pub login_time: DateTime,
    pub server: String,
    #[quassel(name = "ircOperator")]
    pub irc_operator: String,
    // #[quassel(name = "lastAwayMessage")]
    // pub last_away_message: i32,
    #[quassel(name = "lastAwayMessageTime")]
    pub last_away_message_time: DateTime,
    #[quassel(name = "whoisServiceReply")]
    pub whois_service_reply: String,
    #[quassel(name = "suserHost")]
    pub suser_host: String,
    pub encrypted: bool,
    pub channels: StringList,
    #[quassel(name = "userModes")]
    pub user_modes: String,
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

    pub fn update_hostmask(&mut self, mask: String) {}

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
impl crate::message::StatefulSyncableClient for IrcUser {}

#[cfg(feature = "server")]
impl crate::message::StatefulSyncableServer for IrcUser {}

impl Syncable for IrcUser {
    const CLASS: Class = Class::IrcUser;

    fn send_sync(&self, function: &str, params: crate::primitive::VariantList) {
        crate::message::signalproxy::SYNC_PROXY.get().unwrap().sync(
            Self::CLASS,
            None,
            function,
            params,
        );
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
