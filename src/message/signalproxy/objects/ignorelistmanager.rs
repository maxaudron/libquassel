use crate::{
    message::{Class, Syncable},
    primitive::Variant,
};

use libquassel_derive::{sync, NetworkList, NetworkMap};
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Default, Debug, Clone, PartialEq, NetworkList, NetworkMap)]
pub struct IgnoreListManager {
    #[quassel(name = "IgnoreList")]
    #[network(variant = "VariantMap", network = "map")]
    pub ignore_list: Vec<IgnoreListItem>,
}

impl IgnoreListManager {
    /// Get a reference to a specific ignore list by ID.
    pub fn ignore_list_item(&self, rule: &str) -> Option<&IgnoreListItem> {
        if let Some(position) = self
            .ignore_list
            .iter()
            .position(|item| item.ignore_rule.as_str() == rule)
        {
            self.ignore_list.get(position)
        } else {
            None
        }
    }

    /// Get a mutable reference to a specific highlight rule by ID.
    pub fn ignore_list_item_mut(&mut self, rule: &str) -> Option<&mut IgnoreListItem> {
        if let Some(position) = self
            .ignore_list
            .iter()
            .position(|item| item.ignore_rule.as_str() == rule)
        {
            self.ignore_list.get_mut(position)
        } else {
            None
        }
    }

    pub fn request_add_ignore_list_item(
        &self,
        IgnoreListItem {
            ignore_type,
            ignore_rule,
            is_regex,
            strictness,
            scope,
            scope_rule,
            is_active,
        }: IgnoreListItem,
    ) {
        sync!(
            "requestAddIgnoreListItem",
            [
                ignore_type,
                ignore_rule,
                is_regex,
                strictness,
                scope,
                scope_rule,
                is_active
            ]
        )
    }

    pub fn request_remove_ignore_list_item(&self, rule: String) {
        sync!("requestRemoveIgnoreListItem", [rule])
    }

    pub fn request_toggle_ignore_rule(&self, rule: String) {
        sync!("requestToggleIgnoreRule", [rule])
    }

    pub fn add_ignore_list_item(&mut self, item: IgnoreListItem) {
        #[cfg(feature = "server")]
        sync!(
            "addIgnoreListItem",
            [
                item.ignore_type,
                item.ignore_rule.clone(),
                item.is_regex,
                item.strictness,
                item.scope,
                item.scope_rule.clone(),
                item.is_active
            ]
        );

        if self.ignore_list_item(&item.ignore_rule).is_none() {
            self.ignore_list.push(item)
        };
    }

    pub fn remove_ignore_list_item(&mut self, rule: &str) {
        if let Some(position) = self
            .ignore_list
            .iter()
            .position(|item| item.ignore_rule.as_str() == rule)
        {
            self.ignore_list.remove(position);
        };

        #[cfg(feature = "server")]
        sync!("removeIgnoreListItem", [rule])
    }

    pub fn toggle_ignore_rule(&mut self, rule: &str) {
        if let Some(item) = self.ignore_list_item_mut(rule) {
            item.is_active = !item.is_active
        }

        #[cfg(feature = "server")]
        sync!("toggleIgnoreRule", [rule])
    }
}

#[cfg(feature = "client")]
impl crate::message::StatefulSyncableClient for IgnoreListManager {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "addIgnoreListItem" => self.add_ignore_list_item(IgnoreListItem {
                ignore_type: get_param!(msg),
                ignore_rule: get_param!(msg),
                is_regex: get_param!(msg),
                strictness: get_param!(msg),
                scope: get_param!(msg),
                scope_rule: get_param!(msg),
                is_active: get_param!(msg),
            }),
            "removeIgnoreListItem" => {
                let rule: String = get_param!(msg);
                self.remove_ignore_list_item(&rule);
            }
            "toggleIgnoreRule" => {
                let rule: String = get_param!(msg);
                self.toggle_ignore_rule(&rule);
            }
            _ => (),
        }
    }
}

#[cfg(feature = "server")]
impl crate::message::StatefulSyncableServer for IgnoreListManager {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "requestAddIgnoreListItem" => self.add_ignore_list_item(IgnoreListItem {
                ignore_type: get_param!(msg),
                ignore_rule: get_param!(msg),
                is_regex: get_param!(msg),
                strictness: get_param!(msg),
                scope: get_param!(msg),
                scope_rule: get_param!(msg),
                is_active: get_param!(msg),
            }),
            "requestRemoveIgnoreListItem" => {
                let rule: String = get_param!(msg);
                self.remove_ignore_list_item(&rule);
            }
            "requestToggleIgnoreRule" => {
                let rule: String = get_param!(msg);
                self.toggle_ignore_rule(&rule);
            }
            _ => (),
        }
    }
}

impl Syncable for IgnoreListManager {
    const CLASS: Class = Class::IgnoreListManager;
}

#[derive(Debug, Clone, PartialEq, NetworkMap)]
#[network(repr = "maplist")]
pub struct IgnoreListItem {
    #[network(rename = "ignoreType", type = "i32")]
    pub ignore_type: IgnoreType,
    #[network(rename = "ignoreRule", stringlist)]
    pub ignore_rule: String,
    #[network(rename = "isRegEx")]
    pub is_regex: bool,
    #[network(rename = "strictness", type = "i32")]
    pub strictness: StrictnessType,
    #[network(rename = "scope", type = "i32")]
    pub scope: ScopeType,
    #[network(rename = "scopeRule", stringlist)]
    pub scope_rule: String,
    #[network(rename = "isActive")]
    pub is_active: bool,
}

/////////////////////////////////////

//////////////////////////////////////

use num_traits::{FromPrimitive, ToPrimitive};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
pub enum IgnoreType {
    SenderIgnore = 0x00,
    MessageIgnore = 0x01,
    CtcpIgnore = 0x02,
}

impl From<IgnoreType> for Variant {
    fn from(value: IgnoreType) -> Self {
        Variant::i32(value.to_i32().unwrap())
    }
}

impl From<Variant> for IgnoreType {
    fn from(value: Variant) -> Self {
        IgnoreType::from_i32(value.try_into().unwrap()).unwrap()
    }
}

impl From<IgnoreType> for i32 {
    fn from(value: IgnoreType) -> Self {
        value.to_i32().unwrap()
    }
}

impl TryFrom<i32> for IgnoreType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(IgnoreType::SenderIgnore),
            0x01 => Ok(IgnoreType::MessageIgnore),
            0x02 => Ok(IgnoreType::CtcpIgnore),
            _ => Err("no matching IgnoreType found"),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
pub enum StrictnessType {
    UnmatchedStrictness = 0x00,
    SoftStrictness = 0x01,
    HardStrictness = 0x02,
}

impl From<StrictnessType> for Variant {
    fn from(value: StrictnessType) -> Self {
        Variant::i32(value.to_i32().unwrap())
    }
}

impl From<Variant> for StrictnessType {
    fn from(value: Variant) -> Self {
        StrictnessType::from_i32(value.try_into().unwrap()).unwrap()
    }
}

impl From<StrictnessType> for i32 {
    fn from(value: StrictnessType) -> Self {
        value.to_i32().unwrap()
    }
}

impl TryFrom<i32> for StrictnessType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(StrictnessType::UnmatchedStrictness),
            0x01 => Ok(StrictnessType::SoftStrictness),
            0x02 => Ok(StrictnessType::HardStrictness),
            _ => Err("no matching StrictnessType found"),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ScopeType {
    GlobalScope = 0x00,
    NetworkScope = 0x01,
    ChannelScope = 0x02,
}

impl From<ScopeType> for Variant {
    fn from(value: ScopeType) -> Self {
        Variant::i32(value.to_i32().unwrap())
    }
}

impl From<Variant> for ScopeType {
    fn from(value: Variant) -> Self {
        ScopeType::from_i32(value.try_into().unwrap()).unwrap()
    }
}

impl From<ScopeType> for i32 {
    fn from(value: ScopeType) -> Self {
        value.to_i32().unwrap()
    }
}

impl TryFrom<i32> for ScopeType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ScopeType::GlobalScope),
            0x01 => Ok(ScopeType::NetworkScope),
            0x02 => Ok(ScopeType::ChannelScope),
            _ => Err("no matching ScopeType found"),
        }
    }
}
