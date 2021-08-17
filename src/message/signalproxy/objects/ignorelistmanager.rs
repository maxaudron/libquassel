use libquassel_derive::Network;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Network)]
#[network(repr = "list")]
pub struct IgnoreListManager {
    #[network(rename = "IgnoreList", network, variant = "VariantMap")]
    ignore_list: Vec<IgnoreListItem>,
    // // C->S calls

    // requestAddIgnoreListItem(type: Int, ignoreRule: QString,
    //     isRegEx: Bool, strictness: Int, scope: Int, scopeRule: QString,
    //     isActive: Bool)
    // requestRemoveIgnoreListItem(ignoreRule: QString)
    // requestToggleIgnoreRule(ignoreRule: QString)
    // /**
    //  * Replaces all properties of the object with the content of the
    //  * "properties" parameter. This parameter is in network representation.
    //  */
    // requestUpdate(properties: QVariantMap)

    // // S->C calls

    // addIgnoreListItem(type: Int, ignoreRule: QString, isRegEx: Bool,
    //     strictness: Int, scope: Int, scopeRule: QString, isActive: Bool)
    // removeIgnoreListItem(ignoreRule: QString)
    // toggleIgnoreRule(ignoreRule: QString)
    // /**
    //  * Replaces all properties of the object with the content of the
    //  * "properties" parameter. This parameter is in network representation.
    //  */
    // update(properties: QVariantMap)
}

#[derive(Debug, Clone, PartialEq, Network)]
#[network(repr = "maplist")]
pub struct IgnoreListItem {
    #[network(rename = "ignoreType", network, type = "u8")]
    ignore_type: IgnoreType,
    #[network(rename = "ignoreRule")]
    ignore_rule: String,
    #[network(rename = "isRegEx")]
    is_reg_ex: bool,
    #[network(rename = "strictness", network, type = "u8")]
    strictness: StrictnessType,
    #[network(rename = "scope", network, type = "u8")]
    scope: ScopeType,
    #[network(rename = "scopeRule")]
    scope_rule: String,
    #[network(rename = "isActive")]
    is_active: bool,
}

/////////////////////////////////////

//////////////////////////////////////

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IgnoreType {
    SenderIgnore = 0x00,
    MessageIgnore = 0x01,
    CtcpIgnore = 0x02,
}

impl TryFrom<u8> for IgnoreType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(IgnoreType::SenderIgnore),
            0x01 => Ok(IgnoreType::MessageIgnore),
            0x02 => Ok(IgnoreType::CtcpIgnore),
            _ => Err("no matching IgnoreType found"),
        }
    }
}

impl super::Network for IgnoreType {
    type Item = u8;

    fn to_network(&self) -> Self::Item {
        *self as u8
    }

    fn from_network(input: &mut Self::Item) -> Self {
        IgnoreType::try_from(*input).unwrap()
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StrictnessType {
    UnmatchedStrictness = 0x00,
    SoftStrictness = 0x01,
    HardStrictness = 0x02,
}

impl TryFrom<u8> for StrictnessType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(StrictnessType::UnmatchedStrictness),
            0x01 => Ok(StrictnessType::SoftStrictness),
            0x02 => Ok(StrictnessType::HardStrictness),
            _ => Err("no matching StrictnessType found"),
        }
    }
}

impl super::Network for StrictnessType {
    type Item = u8;

    fn to_network(&self) -> Self::Item {
        *self as u8
    }

    fn from_network(input: &mut Self::Item) -> Self {
        Self::try_from(*input).unwrap()
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScopeType {
    GlobalScope = 0x00,
    NetworkScope = 0x01,
    ChannelScope = 0x02,
}

impl TryFrom<u8> for ScopeType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ScopeType::GlobalScope),
            0x01 => Ok(ScopeType::NetworkScope),
            0x02 => Ok(ScopeType::ChannelScope),
            _ => Err("no matching ScopeType found"),
        }
    }
}

impl super::Network for ScopeType {
    type Item = u8;

    fn to_network(&self) -> Self::Item {
        *self as u8
    }

    fn from_network(input: &mut Self::Item) -> Self {
        Self::try_from(*input).unwrap()
    }
}
