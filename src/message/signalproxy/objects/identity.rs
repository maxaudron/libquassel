use libquassel_derive::Network;

#[derive(Debug, Clone, PartialEq, Network)]
#[network(repr = "map")]
pub struct Identity {
    #[network(rename = "identityId")]
    identity_id: i32,
    #[network(rename = "identityName")]
    identity_name: String,
    #[network(rename = "realName")]
    real_name: String,
    #[network(rename = "nicks", type = "StringList")]
    nicks: Vec<String>,
    #[network(rename = "awayNick")]
    away_nick: String,
    #[network(rename = "awayNickEnabled")]
    away_nick_enabled: bool,
    #[network(rename = "awayReason")]
    away_reason: String,
    #[network(rename = "awayReasonEnabled")]
    away_reason_enabled: bool,
    #[network(rename = "autoAwayEnabled")]
    auto_away_enabled: bool,
    #[network(rename = "autoAwayTime")]
    auto_away_time: i32,
    #[network(rename = "autoAwayReason")]
    auto_away_reason: String,
    #[network(rename = "autoAwayReasonEnabled")]
    auto_away_reason_enabled: bool,
    #[network(rename = "detachAwayEnabled")]
    detach_away_enabled: bool,
    #[network(rename = "detachAwayReason")]
    detach_away_reason: String,
    #[network(rename = "detachAwayReasonEnabled")]
    detach_away_reason_enabled: bool,
    #[network(rename = "ident")]
    ident: String,
    #[network(rename = "kickReason")]
    kick_reason: String,
    #[network(rename = "partReason")]
    part_reason: String,
    #[network(rename = "quitReason")]
    quit_reason: String,
}
