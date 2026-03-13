use std::fmt::Display;

use once_cell::sync::OnceCell;

use crate::{FeatureError, ProtocolError, Result, primitive::StringList};

pub static FEATURES: OnceCell<Vec<Feature>> = OnceCell::new();

/// ## Quassel Features
///
/// The quassel protocol implements feature flags to provide a wide range of up and downward compatibility.
/// This enum represents these features.
///
/// When establishing a new connection between core and client, the client first sends it's list of
/// supported features in the [`super::ClientInit`] handshake message, the core then returns it's
/// supported features with [`super::ClientInitAck`]. The features that are common
/// between both are then enabled.
///
/// The default set of features supported by this library are:
/// - "ExtendedFeatures"
/// - "LongMessageId"
/// - "LongTime"
/// - "RichMessages"
/// - "SenderPrefixes"
/// - "Authenticators"
///
/// Any other features will require support from your implementation of client or core
/// and will need to be added by you in the init phase as appropriate.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Feature {
    /// --
    SynchronizedMarkerLine = 0x00000001,
    /// --
    SaslAuthentication = 0x00000002,
    /// --
    SaslExternal = 0x00000004,
    /// --
    HideInactiveNetworks = 0x00000008,
    /// --
    PasswordChange = 0x00000010,
    /// IRCv3 capability negotiation, account tracking
    CapNegotiation = 0x00000020,
    /// IRC server SSL validation
    VerifyServerSSL = 0x00000040,
    /// IRC server custom message rate limits
    CustomRateLimits = 0x00000080,
    /// Currently not supported
    DccFileTransfer = 0x00000100,
    /// Timestamp formatting in away (e.g. %%hh:mm%%)
    AwayFormatTimestamp = 0x00000200,
    /// Support for exchangeable auth backends
    Authenticators = 0x00000400,
    /// Sync buffer activity status
    BufferActivitySync = 0x00000800,
    /// Core-Side highlight configuration and matching
    CoreSideHighlights = 0x00001000,
    /// Show prefixes for senders in backlog
    SenderPrefixes = 0x00002000,
    /// Supports RPC call disconnectFromCore to remotely disconnect a client
    RemoteDisconnect = 0x00004000,
    /// Transmit features as list of strings
    ExtendedFeatures = 0x00008000,
    /// Serialize message time as 64-bit
    LongTime,
    /// Real Name and Avatar URL in backlog
    RichMessages,
    /// Backlogmanager supports filtering backlog by messagetype
    BacklogFilterType,
    /// ECDSA keys for CertFP in identities
    EcdsaCertfpKeys,
    /// 64-bit IDs for messages
    LongMessageId,
    /// CoreInfo dynamically updated using signals
    SyncedCoreInfo,
}

impl Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Feature::SynchronizedMarkerLine => f.write_str("SynchronizedMarkerLine"),
            Feature::SaslAuthentication => f.write_str("SaslAuthentication"),
            Feature::SaslExternal => f.write_str("SaslExternal"),
            Feature::HideInactiveNetworks => f.write_str("HideInactiveNetworks"),
            Feature::PasswordChange => f.write_str("PasswordChange"),
            Feature::CapNegotiation => f.write_str("CapNegotiation"),
            Feature::VerifyServerSSL => f.write_str("VerifyServerSSL"),
            Feature::CustomRateLimits => f.write_str("CustomRateLimits"),
            Feature::DccFileTransfer => f.write_str("DccFileTransfer"),
            Feature::AwayFormatTimestamp => f.write_str("AwayFormatTimestamp"),
            Feature::Authenticators => f.write_str("Authenticators"),
            Feature::BufferActivitySync => f.write_str("BufferActivitySync"),
            Feature::CoreSideHighlights => f.write_str("CoreSideHighlights"),
            Feature::SenderPrefixes => f.write_str("SenderPrefixes"),
            Feature::RemoteDisconnect => f.write_str("RemoteDisconnect"),
            Feature::ExtendedFeatures => f.write_str("ExtendedFeatures"),
            Feature::LongTime => f.write_str("LongTime"),
            Feature::RichMessages => f.write_str("RichMessages"),
            Feature::BacklogFilterType => f.write_str("BacklogFilterType"),
            Feature::EcdsaCertfpKeys => f.write_str("EcdsaCertfpKeys"),
            Feature::LongMessageId => f.write_str("LongMessageId"),
            Feature::SyncedCoreInfo => f.write_str("SyncedCoreInfo"),
        }
    }
}

impl std::str::FromStr for Feature {
    type Err = ProtocolError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "SynchronizedMarkerLine" => Self::SynchronizedMarkerLine,
            "SaslAuthentication" => Self::SaslAuthentication,
            "SaslExternal" => Self::SaslExternal,
            "HideInactiveNetworks" => Self::HideInactiveNetworks,
            "PasswordChange" => Self::PasswordChange,
            "CapNegotiation" => Self::CapNegotiation,
            "VerifyServerSSL" => Self::VerifyServerSSL,
            "CustomRateLimits" => Self::CustomRateLimits,
            "DccFileTransfer" => Self::DccFileTransfer,
            "AwayFormatTimestamp" => Self::AwayFormatTimestamp,
            "Authenticators" => Self::Authenticators,
            "BufferActivitySync" => Self::BufferActivitySync,
            "CoreSideHighlights" => Self::CoreSideHighlights,
            "SenderPrefixes" => Self::SenderPrefixes,
            "RemoteDisconnect" => Self::RemoteDisconnect,
            "ExtendedFeatures" => Self::ExtendedFeatures,
            "LongTime" => Self::LongTime,
            "RichMessages" => Self::RichMessages,
            "BacklogFilterType" => Self::BacklogFilterType,
            "EcdsaCertfpKeys" => Self::EcdsaCertfpKeys,
            "LongMessageId" => Self::LongMessageId,
            "SyncedCoreInfo" => Self::SyncedCoreInfo,
            unknown => return Err(ProtocolError::UnknownFeature(unknown.to_string())),
        })
    }
}

impl Feature {
    pub fn get() -> StringList {
        vec![
            s!("ExtendedFeatures"),
            s!("LongMessageId"),
            s!("LongTime"),
            s!("RichMessages"),
            s!("SenderPrefixes"),
            s!("Authenticators"),
        ]
    }

    pub fn enable_all() -> Result<()> {
        FEATURES
            .set(vec![
                Feature::ExtendedFeatures,
                Feature::LongMessageId,
                Feature::LongTime,
                Feature::RichMessages,
                Feature::SenderPrefixes,
                Feature::Authenticators,
            ])
            .map_err(|_| FeatureError::AlreadyInitialized)?;
        Ok(())
    }

    pub fn enabled(self) -> Result<bool> {
        let features = FEATURES.get().ok_or(FeatureError::NotInitialized)?;
        Ok(features.contains(&self))
    }
}
