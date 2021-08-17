use libquassel_derive::{Network, NetworkList, NetworkMap};

use crate::message::{StatefulSyncable, SyncProxy, Syncable};

use crate::message::signalproxy::translation::Network;
use crate::primitive::{VariantList, VariantMap};

/// AliasManager
/// keeps a list of all registered aliases
/// syncable
#[derive(Clone, Debug, std::cmp::PartialEq, NetworkList, NetworkMap)]
#[network]
pub struct AliasManager {
    #[network(rename = "Aliases", variant = "VariantMap", network)]
    pub aliases: Vec<Alias>,
}

impl AliasManager {
    pub fn add_alias(&mut self, alias: Alias) {
        // TODO check if name is equal
        if !self.aliases.contains(&alias) {
            self.aliases.push(alias)
        }
    }
}

impl StatefulSyncable for AliasManager {}
impl Syncable for AliasManager {
    fn sync(&self, session: impl SyncProxy, function: &str, params: crate::primitive::VariantList) {
        session.sync("AliasManager", None, function, params)
    }
}

/// Alias
/// Represents a signle alias
#[derive(Clone, Debug, std::cmp::PartialEq, Network)]
#[network(repr = "maplist")]
pub struct Alias {
    #[network(rename = "names", variant = "StringList")]
    name: String,
    #[network(rename = "expansions", variant = "StringList")]
    expansion: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::signalproxy::translation::Network;

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
        assert_eq!(get_src().to_network(), get_dest())
    }

    #[test]
    fn aliasmanager_from_network() {
        assert_eq!(AliasManager::from_network(&mut get_dest()), get_src())
    }
}
