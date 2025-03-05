#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Class {
    AliasManager,
    BacklogManager,
    BufferSyncer,
    BufferViewConfig,
    BufferViewManager,
    CoreInfo,
    CoreData,
    HighlightRuleManager,
    Identity,
    IgnoreListManager,
    CertManager,
    Network,
    NetworkInfo,
    NetworkConfig,
    IrcChannel,
    IrcUser,
    Unknown,
}

impl From<String> for Class {
    fn from(class: String) -> Self {
        Self::from(class.as_str())
    }
}

impl From<&str> for Class {
    fn from(class: &str) -> Self {
        match class {
            "AliasManager" => Self::AliasManager,
            "BacklogManager" => Class::BacklogManager,
            "BufferSyncer" => Self::BufferSyncer,
            "BufferViewConfig" => Self::BufferViewConfig,
            "BufferViewManager" => Self::BufferViewManager,
            "CoreInfo" => Self::CoreInfo,
            "CoreData" => Self::CoreData,
            "HighlightRuleManager" => Self::HighlightRuleManager,
            "Identity" => Self::Identity,
            "IgnoreListManager" => Self::IgnoreListManager,
            "CertManager" => Self::CertManager,
            "Network" => Self::Network,
            "NetworkInfo" => Self::NetworkInfo,
            "NetworkConfig" => Self::NetworkConfig,
            "IrcChannel" => Self::IrcChannel,
            "IrcUser" => Self::IrcUser,
            _ => Self::Unknown,
        }
    }
}

impl Class {
    pub fn as_str(&self) -> &str {
        match self {
            Class::AliasManager => "AliasManager",
            Class::BacklogManager => "BacklogManager",
            Class::BufferSyncer => "BufferSyncer",
            Class::BufferViewConfig => "BufferViewConfig",
            Class::BufferViewManager => "BufferViewManager",
            Class::CoreInfo => "CoreInfo",
            Class::CoreData => "CoreData",
            Class::HighlightRuleManager => "HighlightRuleManager",
            Class::Identity => "Identity",
            Class::IgnoreListManager => "IgnoreListManager",
            Class::CertManager => "CertManager",
            Class::Network => "Network",
            Class::NetworkInfo => "NetworkInfo",
            Class::NetworkConfig => "NetworkConfig",
            Class::IrcChannel => "IrcChannel",
            Class::IrcUser => "IrcUser",
            Class::Unknown => "Unknown",
        }
    }
}
