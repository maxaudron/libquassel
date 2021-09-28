use druid::{Selector, SingleUse};
use libquassel::{
    message::{
        objects::{Alias, AliasManager},
        SyncMessage,
    },
    primitive::VariantMap,
};

pub const CONNECT: Selector = Selector::new("connect");
pub const ADD_MESSAGE: Selector<SingleUse<crate::Message>> = Selector::new("add_message");

pub const ALIASMANAGER_INIT: Selector<SingleUse<AliasManager>> = Selector::new("aliasmanager_init");
pub const ALIASMANAGER_UPDATE: Selector<SingleUse<SyncMessage>> =
    Selector::new("aliasmanager_update");
pub const ALIASMANAGER_ADD_ALIAS: Selector<SingleUse<Alias>> =
    Selector::new("aliasmanager_add_alias");
