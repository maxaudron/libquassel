use std::collections::HashMap;

use crate::{
    message::NetworkMap,
    primitive::{StringList, Variant, VariantMap},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ChanModes {
    /// Modes that add or remove items from a list, like commonly +b for the banlist.
    ///
    /// Always require a parameter from server to client.
    /// Clients can request the whole list by leaving the parameter empty
    pub channel_modes_a: HashMap<char, StringList>,

    /// Modes that take a parameter as setting and require it when setting or removing the mode.
    pub channel_modes_b: HashMap<char, String>,

    /// Modes that take a parameter as setting, but only require it when setting the mode.
    pub channel_modes_c: HashMap<char, String>,

    /// Modes without a parameter.
    pub channel_modes_d: String,
}

impl NetworkMap for ChanModes {
    type Item = VariantMap;

    fn to_network_map(&self) -> Self::Item {
        let mut map = VariantMap::new();

        map.insert(
            s!("A"),
            Variant::VariantMap(
                self.channel_modes_a
                    .iter()
                    .map(|(k, v)| (k.to_string(), Variant::StringList(v.clone())))
                    .collect(),
            ),
        );
        map.insert(
            s!("B"),
            Variant::VariantMap(
                self.channel_modes_b
                    .iter()
                    .map(|(k, v)| (k.to_string(), Variant::String(v.clone())))
                    .collect(),
            ),
        );
        map.insert(
            s!("C"),
            Variant::VariantMap(
                self.channel_modes_c
                    .iter()
                    .map(|(k, v)| (k.to_string(), Variant::String(v.clone())))
                    .collect(),
            ),
        );
        map.insert(s!("D"), Variant::String(self.channel_modes_d.clone()));

        map
    }

    fn from_network_map(input: &mut Self::Item) -> Self {
        ChanModes {
            channel_modes_a: match_variant!(input.remove("A").unwrap(), Variant::VariantMap)
                .into_iter()
                .map(|(mut k, v)| (k.remove(0), match_variant!(v, Variant::StringList)))
                .collect(),
            channel_modes_b: match_variant!(input.remove("B").unwrap(), Variant::VariantMap)
                .into_iter()
                .map(|(mut k, v)| (k.remove(0), match_variant!(v, Variant::String)))
                .collect(),
            channel_modes_c: match_variant!(input.remove("C").unwrap(), Variant::VariantMap)
                .into_iter()
                .map(|(mut k, v)| (k.remove(0), match_variant!(v, Variant::String)))
                .collect(),
            channel_modes_d: match_variant!(input.remove("D").unwrap(), Variant::String),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_network() -> VariantMap {
        map! {
            s!("B") => Variant::VariantMap(map!
                {},
            ),
            s!("D") => Variant::String(
                s!("tCnT"),
            ),
            s!("C") => Variant::VariantMap(map!
                {
                    s!("j") => Variant::String(
                        s!("5:1"),
                    ),
                    s!("x") => Variant::String(
                        s!("10:5"),
                    ),
                    s!("f") => Variant::String(
                        s!("30:5"),
                    ),
                    s!("F") => Variant::String(
                        s!("5:60"),
                    ),
                },
            ),
            s!("A") => Variant::VariantMap(map! {
                s!("b") => Variant::StringList(vec![s!("*!*@test"), s!("*!*@test2")]),
            }),
        }
    }

    fn get_runtime() -> ChanModes {
        ChanModes {
            channel_modes_a: map! { 'b' => vec![s!("*!*@test"), s!("*!*@test2")] },
            channel_modes_b: map! {},
            channel_modes_c: map! { 'j' => s!("5:1"), 'x' => s!("10:5"), 'f' => s!("30:5"), 'F' => s!("5:60") },
            channel_modes_d: s!("tCnT"),
        }
    }

    #[test]
    fn chanmodes_to_network() {
        assert_eq!(get_runtime().to_network_map(), get_network())
    }

    #[test]
    fn chanmodes_from_network() {
        assert_eq!(ChanModes::from_network_map(&mut get_network()), get_runtime())
    }
}
