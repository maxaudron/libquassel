#[allow(unused_imports)]
use libquassel_derive::sync;
use libquassel_derive::{NetworkList, NetworkMap};

#[allow(unused_imports)]
use crate::message::signalproxy::translation::NetworkMap;
use crate::message::Class;
#[allow(unused_imports)]
use crate::message::StatefulSyncableClient;
#[allow(unused_imports)]
use crate::message::StatefulSyncableServer;

use crate::message::Syncable;

#[allow(unused_imports)]
use crate::primitive::VariantMap;
use crate::Result;

/// AliasManager
/// keeps a list of all registered aliases
/// syncable
#[derive(Clone, Default, Debug, std::cmp::PartialEq, NetworkList, NetworkMap)]
pub struct AliasManager {
    #[network(rename = "Aliases", variant = "VariantMap", network = "map")]
    pub aliases: Vec<Alias>,
}

impl AliasManager {
    pub fn add_alias(&mut self, alias: Alias) -> Result<()> {
        #[cfg(feature = "server")]
        sync!("addAlias", [alias.to_network_map()?])?;

        if !self.aliases.contains(&alias) {
            self.aliases.push(alias)
        }

        Ok(())
    }
}

#[cfg(feature = "client")]
impl StatefulSyncableClient for AliasManager {}

#[cfg(feature = "server")]
impl StatefulSyncableServer for AliasManager {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage) -> crate::Result<()>
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "addAlias" => self.add_alias(Alias::from_network_map(&mut VariantMap::try_from(
                msg.params
                    .pop()
                    .ok_or(crate::ProtocolError::MissingSyncMessageParams)?,
            )?)?),
            unknown => Err(crate::ProtocolError::UnknownMsgSlotName(unknown.to_string())),
        }
    }
}

impl Syncable for AliasManager {
    const CLASS: Class = Class::AliasManager;
}

/// Alias
/// Represents a signle alias
#[derive(Clone, Debug, std::cmp::PartialEq, NetworkMap)]
#[network(repr = "maplist")]
pub struct Alias {
    #[network(rename = "names", stringlist)]
    pub name: String,
    #[network(rename = "expansions", stringlist)]
    pub expansion: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::signalproxy::translation::NetworkList;

    use crate::primitive::{Variant, VariantList};

    fn get_src() -> AliasManager {
        AliasManager {
            aliases: vec![
                Alias {
                    name: s!("j"),
                    expansion: s!("/join $0"),
                },
                Alias {
                    name: s!("ns"),
                    expansion: s!("/msg nickserv $0"),
                },
            ],
        }
    }

    fn get_dest() -> VariantList {
        vec![
            Variant::ByteArray(s!("Aliases")),
            Variant::VariantMap(map! {
                s!("names") => Variant::StringList(
                    vec![
                        s!("j"),
                        s!("ns"),
                    ],
                ),
                s!("expansions") => Variant::StringList(
                    vec![
                        s!("/join $0"),
                        s!("/msg nickserv $0"),
                    ],
                ),
            }),
        ]
    }

    // #[bench]
    // fn alias_to_network(b: &mut test::Bencher) {
    //     b.iter(|| test::black_box(get_src()).to_network())
    // }

    // #[bench]
    // fn alias_from_network(b: &mut test::Bencher) {
    //     b.iter(|| AliasManager::from_network(&mut test::black_box(get_dest())))
    // }

    #[test]
    fn aliasmanager_to_network() {
        assert_eq!(get_src().to_network_list().unwrap(), get_dest())
    }

    #[test]
    fn aliasmanager_from_network() {
        assert_eq!(
            AliasManager::from_network_list(&mut get_dest()).unwrap(),
            get_src()
        )
    }
}
