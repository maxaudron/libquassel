#[allow(unused_imports)]
use libquassel_derive::sync;
use libquassel_derive::{NetworkList, NetworkMap, Setters};

use crate::message::Class;
#[allow(unused_imports)]
use crate::message::StatefulSyncableClient;
#[allow(unused_imports)]
use crate::message::StatefulSyncableServer;

use crate::message::Syncable;

#[allow(unused_imports)]
use crate::message::signalproxy::translation::NetworkMap;
use crate::primitive::IdentityId;
use crate::primitive::VariantMap;
use crate::serialize::Deserialize;
use crate::serialize::Serialize;
use crate::serialize::UserType;
use crate::{Result, SyncProxyError};

#[derive(Default, Debug, Clone, PartialEq, NetworkMap, NetworkList, Setters)]
pub struct Identity {
    #[network(rename = "identityId")]
    pub identity_id: IdentityId,
    #[network(rename = "identityName")]
    pub identity_name: String,
    #[network(rename = "realName")]
    pub real_name: String,
    #[network(rename = "nicks")]
    #[network(type = "StringList")]
    pub nicks: Vec<String>,

    /// Away Nick is not actually used
    /// in official network client
    #[network(rename = "awayNick")]
    pub away_nick: String,
    #[network(rename = "awayNickEnabled")]
    pub away_nick_enabled: bool,

    #[network(rename = "awayReason")]
    pub away_reason: String,
    #[network(rename = "awayReasonEnabled")]
    pub away_reason_enabled: bool,
    #[network(rename = "autoAwayEnabled")]
    pub auto_away_enabled: bool,
    #[network(rename = "autoAwayTime")]
    pub auto_away_time: i32,
    #[network(rename = "autoAwayReason")]
    pub auto_away_reason: String,
    #[network(rename = "autoAwayReasonEnabled")]
    pub auto_away_reason_enabled: bool,
    #[network(rename = "detachAwayEnabled")]
    pub detach_away_enabled: bool,
    #[network(rename = "detachAwayReason")]
    pub detach_away_reason: String,
    #[network(rename = "detachAwayReasonEnabled")]
    pub detach_away_reason_enabled: bool,
    #[network(rename = "ident")]
    pub ident: String,
    #[network(rename = "kickReason")]
    pub kick_reason: String,
    #[network(rename = "partReason")]
    pub part_reason: String,
    #[network(rename = "quitReason")]
    pub quit_reason: String,
}

impl UserType for Identity {
    const NAME: &str = "Identity";
}

impl Serialize for Identity {
    fn serialize(&self) -> Result<Vec<u8>> {
        self.to_network_map().serialize()
    }
}

impl Deserialize for Identity {
    fn parse(b: &[u8]) -> Result<(usize, Self)>
    where
        Self: std::marker::Sized,
    {
        let (vlen, mut value) = VariantMap::parse(b)?;
        Ok((vlen, Self::from_network_map(&mut value)))
    }
}

impl Identity {
    pub fn copy_from(&mut self, other: Identity) -> Result<()> {
        #[cfg(feature = "server")]
        sync!("copyFrom", [other.to_network_map()])?;

        *self = other;

        Ok(())
    }
}

#[cfg(feature = "client")]
impl StatefulSyncableClient for Identity {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage) -> Result<()>
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "copyFrom" => self.copy_from(Identity::from_network_map(&mut get_param!(msg))),
            "setAutoAwayEnabled" => self.set_auto_away_enabled(get_param!(msg)),
            "setAutoAwayReason" => self.set_auto_away_reason(get_param!(msg)),
            "setAutoAwayReasonEnabled" => self.set_auto_away_reason_enabled(get_param!(msg)),
            "setAutoAwayTime" => self.set_auto_away_time(get_param!(msg)),
            "setAwayNick" => self.set_away_nick(get_param!(msg)),
            "setAwayNickEnabled" => self.set_away_nick_enabled(get_param!(msg)),
            "setAwayReason" => self.set_away_reason(get_param!(msg)),
            "setAwayReasonEnabled" => self.set_away_reason_enabled(get_param!(msg)),
            "setDetachAwayEnabled" => self.set_detach_away_enabled(get_param!(msg)),
            "setDetachAwayReason" => self.set_detach_away_reason(get_param!(msg)),
            "setDetachAwayReasonEnabled" => self.set_detach_away_reason_enabled(get_param!(msg)),
            "setId" => self.set_identity_id(get_param!(msg)),
            "setIdent" => self.set_ident(get_param!(msg)),
            "setIdentityName" => self.set_identity_name(get_param!(msg)),
            "setKickReason" => self.set_kick_reason(get_param!(msg)),
            "setNicks" => self.set_nicks(get_param!(msg)),
            "setPartReason" => self.set_part_reason(get_param!(msg)),
            "setQuitReason" => self.set_quit_reason(get_param!(msg)),
            "setRealName" => self.set_real_name(get_param!(msg)),
            unknown => Err(crate::ProtocolError::UnknownMsgSlotName(unknown.to_string())),
        }
    }
}

#[cfg(feature = "server")]
impl StatefulSyncableServer for Identity {}

impl Syncable for Identity {
    const CLASS: Class = Class::Identity;

    fn send_sync(&self, function: &str, params: crate::primitive::VariantList) -> crate::Result<()> {
        crate::message::signalproxy::SYNC_PROXY
            .get()
            .ok_or(SyncProxyError::NotInitialized)?
            .sync(Self::CLASS, Some(&self.identity_id.to_string()), function, params)
    }
}
