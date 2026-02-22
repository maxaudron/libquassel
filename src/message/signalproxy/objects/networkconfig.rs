use libquassel_derive::{NetworkList, NetworkMap, Setters};

use crate::message::{Class, Syncable};

#[derive(Debug, Default, Clone, PartialEq, NetworkList, NetworkMap, Setters)]
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

impl Syncable for NetworkConfig {
    const CLASS: Class = Class::NetworkConfig;
}

#[cfg(feature = "client")]
impl crate::message::StatefulSyncableClient for NetworkConfig {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage) -> Result<(), crate::error::ProtocolError>
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "setAutoWhoDelay" => self.set_auto_who_delay(get_param!(msg)),
            "setAutoWhoEnabled" => self.set_auto_who_enabled(get_param!(msg)),
            "setAutoWhoInterval" => self.set_auto_who_interval(get_param!(msg)),
            "setAutoWhoNickLimit" => self.set_auto_who_nick_limit(get_param!(msg)),
            "setMaxPingCount" => self.set_max_ping_count(get_param!(msg)),
            "setPingInterval" => self.set_ping_interval(get_param!(msg)),
            "setPingTimeoutEnabled" => self.set_ping_timeout_enabled(get_param!(msg)),
            "setStandardCtcp" => self.set_standard_ctcp(get_param!(msg)),
            _ => Ok(()),
        }
    }
}

#[cfg(feature = "server")]
impl crate::message::StatefulSyncableServer for NetworkConfig {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage) -> Result<(), crate::error::ProtocolError>
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "requestSetAutoWhoDelay" => self.set_auto_who_delay(get_param!(msg)),
            "requestSetAutoWhoEnabled" => self.set_auto_who_enabled(get_param!(msg)),
            "requestSetAutoWhoInterval" => self.set_auto_who_interval(get_param!(msg)),
            "requestSetAutoWhoNickLimit" => self.set_auto_who_nick_limit(get_param!(msg)),
            "requestSetMaxPingCount" => self.set_max_ping_count(get_param!(msg)),
            "requestSetPingInterval" => self.set_ping_interval(get_param!(msg)),
            "requestSetPingTimeoutEnabled" => self.set_ping_timeout_enabled(get_param!(msg)),
            "requestSetStandardCtcp" => self.set_standard_ctcp(get_param!(msg)),
            _ => Ok(()),
        }
    }
}
