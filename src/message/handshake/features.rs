use once_cell::sync::OnceCell;

use crate::{FeatureError, Result, primitive::StringList};

pub static FEATURES: OnceCell<Vec<Feature>> = OnceCell::new();

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

impl Feature {
    pub fn get() -> StringList {
        vec![
            "ExtendedFeatures".to_string(),
            #[cfg(feature = "long-message-id")]
            "LongMessageId".to_string(),
            #[cfg(feature = "long-time")]
            "LongTime".to_string(),
            #[cfg(feature = "rich-messages")]
            "RichMessages".to_string(),
            #[cfg(feature = "sender-prefixes")]
            "SenderPrefixes".to_string(),
            #[cfg(feature = "authenticators")]
            "Authenticators".to_string(),
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
