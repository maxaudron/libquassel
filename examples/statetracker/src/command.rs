use druid::Selector;
use libquassel::message::objects::{Alias, AliasManager};

pub const CONNECT: Selector = Selector::new("connect");
pub const ADD_MESSAGE: Selector<crate::Message> = Selector::new("add_message");

pub const ALIASMANAGER_INIT: Selector<AliasManager> = Selector::new("aliasmanager_init");
pub const ALIASMANAGER_ADD_ALIAS: Selector<Alias> = Selector::new("aliasmanager_add_alias");
