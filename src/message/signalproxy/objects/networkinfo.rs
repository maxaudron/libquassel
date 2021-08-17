use crate::primitive::StringList;

use libquassel_derive::Network;

use crate::message::objects::network::NetworkServer;

#[derive(Debug, Clone, PartialEq, Network)]
#[network(repr = "list")]
pub struct NetworkInfo {
    #[network(rename = "networkName")]
    pub network_name: String,

    #[network(rename = "ServerList", network, variant = "VariantList")]
    pub server_list: Vec<NetworkServer>,
    #[network(rename = "perform")]
    pub perform: StringList,

    #[network(rename = "autoIdentifyService")]
    pub auto_identify_service: String,
    #[network(rename = "autoIdentifyPassword")]
    pub auto_identify_password: String,

    #[network(rename = "saslAccount")]
    pub sasl_account: String,
    #[network(rename = "saslPassword")]
    pub sasl_password: String,

    // ByteArray
    #[network(rename = "codecForServer", type = "ByteArray")]
    pub codec_for_server: String,
    #[network(rename = "codecForEncoding", type = "ByteArray")]
    pub codec_for_encoding: String,
    #[network(rename = "codecForDecoding", type = "ByteArray")]
    pub codec_for_decoding: String,

    // TODO add these type aliases or usertypes in variants
    // pub network_id: NetworkId,
    // pub identity_id: IdentityId,
    #[network(rename = "msgRateBurstSize")]
    pub msg_rate_burst_size: u32,
    #[network(rename = "msgRateMessageDelay")]
    pub msg_rate_message_delay: u32,

    #[network(rename = "autoReconnectInterval")]
    pub auto_reconnect_interval: u32,
    #[network(rename = "autoReconnectRetries")]
    pub auto_reconnect_retries: u16,

    #[network(rename = "rejoinChannels")]
    pub rejoin_channels: bool,
    #[network(rename = "useRandomServer")]
    pub use_random_server: bool,
    #[network(rename = "useAutoIdentify")]
    pub use_auto_identify: bool,
    #[network(rename = "useSasl")]
    pub use_sasl: bool,
    #[network(rename = "useAutoReconnect")]
    pub use_auto_reconnect: bool,
    #[network(rename = "unlimitedReconnectRetries")]
    pub unlimited_reconnect_retries: bool,
    #[network(rename = "useCustomMessageRate")]
    pub use_custom_message_rate: bool,
    #[network(rename = "unlimitedMessageRate")]
    pub unlimited_message_rate: bool,
    // #[network(rename = "autoAwayActive")]
    // pub auto_away_active: bool,
}

#[cfg(test)]
mod tests {
    use crate::primitive::{Variant, VariantList};

    use super::*;
    use crate::message::signalproxy::translation::Network;

    use pretty_assertions::assert_eq;

    fn get_network() -> VariantList {
        vec![
            Variant::ByteArray(s!("networkName")),
            Variant::String(s!("snoonet")),
            Variant::ByteArray(s!("ServerList")),
            Variant::VariantList(vec![]),
            Variant::ByteArray(s!("perform")),
            Variant::StringList(vec![s!("")]),
            Variant::ByteArray(s!("autoIdentifyService")),
            Variant::String(s!("NickServ")),
            Variant::ByteArray(s!("autoIdentifyPassword")),
            Variant::String(s!("")),
            Variant::ByteArray(s!("saslAccount")),
            Variant::String(s!("")),
            Variant::ByteArray(s!("saslPassword")),
            Variant::String(s!("")),
            Variant::ByteArray(s!("codecForServer")),
            Variant::ByteArray(s!("")),
            Variant::ByteArray(s!("codecForEncoding")),
            Variant::ByteArray(s!("")),
            Variant::ByteArray(s!("codecForDecoding")),
            Variant::ByteArray(s!("")),
            Variant::ByteArray(s!("msgRateBurstSize")),
            Variant::u32(5),
            Variant::ByteArray(s!("msgRateMessageDelay")),
            Variant::u32(2200),
            Variant::ByteArray(s!("autoReconnectInterval")),
            Variant::u32(60),
            Variant::ByteArray(s!("autoReconnectRetries")),
            Variant::u16(20),
            Variant::ByteArray(s!("rejoinChannels")),
            Variant::bool(true),
            Variant::ByteArray(s!("useRandomServer")),
            Variant::bool(false),
            Variant::ByteArray(s!("useAutoIdentify")),
            Variant::bool(false),
            Variant::ByteArray(s!("useSasl")),
            Variant::bool(false),
            Variant::ByteArray(s!("useAutoReconnect")),
            Variant::bool(true),
            Variant::ByteArray(s!("unlimitedReconnectRetries")),
            Variant::bool(false),
            Variant::ByteArray(s!("useCustomMessageRate")),
            Variant::bool(false),
            Variant::ByteArray(s!("unlimitedMessageRate")),
            Variant::bool(false),
        ]
    }

    fn get_runtime() -> NetworkInfo {
        NetworkInfo {
            network_name: s!("snoonet"),
            server_list: vec![],
            perform: vec![s!("")],
            auto_identify_service: s!("NickServ"),
            auto_identify_password: s!(""),
            sasl_account: s!(""),
            sasl_password: s!(""),
            codec_for_server: s!(""),
            codec_for_encoding: s!(""),
            codec_for_decoding: s!(""),
            msg_rate_burst_size: 5,
            msg_rate_message_delay: 2200,
            auto_reconnect_interval: 60,
            auto_reconnect_retries: 20,
            rejoin_channels: true,
            use_random_server: false,
            use_auto_identify: false,
            use_sasl: false,
            use_auto_reconnect: true,
            unlimited_reconnect_retries: false,
            use_custom_message_rate: false,
            unlimited_message_rate: false,
            // auto_away_active: (),
        }
    }

    #[test]
    fn networkinfo_to_network() {
        assert_eq!(get_runtime().to_network(), get_network())
    }

    #[test]
    fn networkinfo_from_network() {
        assert_eq!(NetworkInfo::from_network(&mut get_network()), get_runtime())
    }
}
