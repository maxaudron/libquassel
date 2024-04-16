use crate::error::ProtocolError;
use crate::message::MessageType;
use crate::primitive::{Variant, VariantList};
use crate::{deserialize::Deserialize, serialize::Serialize};

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

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct SyncMessage {
    pub class_name: Class,
    pub object_name: String,
    pub slot_name: String,
    pub params: VariantList,
}

// impl Act for SyncMessage {}

impl Serialize for SyncMessage {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, ProtocolError> {
        let mut res = VariantList::new();

        res.push(Variant::i32(MessageType::SyncMessage as i32));
        res.push(Variant::ByteArray(self.class_name.as_str().to_owned()));
        res.push(Variant::ByteArray(self.object_name.clone()));
        res.push(Variant::ByteArray(self.slot_name.clone()));

        res.append(&mut self.params.clone());

        res.serialize()
    }
}

impl Deserialize for SyncMessage {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), ProtocolError> {
        let (size, mut res) = VariantList::parse(&b)?;

        res.remove(0);

        Ok((
            size,
            Self {
                class_name: Class::from(match_variant!(res.remove(0), Variant::ByteArray)),
                object_name: match_variant!(res.remove(0), Variant::ByteArray),
                slot_name: match_variant!(res.remove(0), Variant::ByteArray),
                params: res,
            },
        ))
    }
}
