use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::network::{gen_type, get_field_type};

use super::NetworkField;

pub(crate) fn to(fields: &[NetworkField]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            if !field.skip {
                let field_rename = match &field.rename {
                    Some(name) => name.clone(),
                    None => format!("{}", field.ident.as_ref().unwrap()),
                };

                let field_name = field.ident.as_ref().unwrap();

                let field_inner = match field.network {
                    crate::network::NetworkRepr::List => quote! {
                        self.#field_name.to_network_list().into()
                    },
                    crate::network::NetworkRepr::Map => quote! {
                        self.#field_name.to_network_map().into()
                    },
                    crate::network::NetworkRepr::None => quote! {
                        self.#field_name.clone().into()
                    },
                };

                if field.stringlist {
                    quote! {
                        res.insert(#field_rename.to_string(),
                            libquassel::primitive::Variant::StringList(
                                std::vec::from_elem(#field_inner, 1)));
                    }
                } else {
                    quote! {
                        res.insert(#field_rename.to_string(),
                            libquassel::primitive::Variant::VariantList(
                                std::vec::from_elem(#field_inner, 1)));
                    }
                }
            } else {
                quote! {}
            }
        })
        .collect()
}

pub(crate) fn to_vec(_type_name: &Ident, fields: &[NetworkField]) -> TokenStream {
    let (lists, for_each_inner, map_inserts): (
        Vec<TokenStream>,
        Vec<TokenStream>,
        Vec<TokenStream>,
    ) = fields.iter().fold(
        (Vec::new(), Vec::new(), Vec::new()),
        |(mut lists, mut for_each_inner, mut map_inserts), field| {
            if !field.skip {
                let field_rename = match &field.rename {
                    Some(name) => name.clone(),
                    None => format!("{}", field.ident.as_ref().unwrap()),
                };

                let field_name = field.ident.as_ref().unwrap();

                let field_type = match field.network {
                    crate::network::NetworkRepr::List => gen_type("VariantList"),
                    crate::network::NetworkRepr::Map => gen_type("VariantMap"),
                    crate::network::NetworkRepr::None => get_field_type(field),
                };

                let field_inner = match field.network {
                    crate::network::NetworkRepr::List => quote! {
                        item.#field_name.to_network_list().into()
                    },
                    crate::network::NetworkRepr::Map => quote! {
                        item.#field_name.to_network_map().into()
                    },
                    crate::network::NetworkRepr::None => quote! {
                        item.#field_name.clone().into()
                    },
                };

                // FIXME this section doesn't make any sense to me anymore
                // why do we select a StringList if we have configured a variant???
                if field.stringlist {
                    lists.push(quote! {
                        let mut #field_name: libquassel::primitive::StringList = Vec::with_capacity(self.len());
                    });

                    for_each_inner.push(quote! {
                        #field_name.push(#field_inner);
                    });

                    map_inserts.push(quote! {
                        map.insert(String::from(#field_rename), libquassel::primitive::Variant::StringList(#field_name));
                    });
                } else {
                    lists.push(quote! {
                        let mut #field_name: libquassel::primitive::VariantList = Vec::with_capacity(self.len());
                    });

                    for_each_inner.push(quote! {
                        #field_name.push(libquassel::primitive::Variant::#field_type(#field_inner));
                    });

                    map_inserts.push(quote! {
                        map.insert(String::from(#field_rename), libquassel::primitive::Variant::VariantList(#field_name));
                    });
                }
            }

            (lists, for_each_inner, map_inserts)
        },
    );

    quote! {
        #(#lists)*

        let mut map = libquassel::primitive::VariantMap::new();

        self.iter().for_each(|item| {
            #(#for_each_inner)*
        });

        #(#map_inserts)*

        return map;
    }
}

pub(crate) fn from(fields: &[NetworkField]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();

            let unwrap = if field.default {
                quote! { unwrap_or_default() }
            } else {
                quote! { unwrap() }
            };

            let field_rename = match &field.rename {
                Some(name) => name.clone(),
                None => format!("{}", field.ident.as_ref().unwrap()),
            };

            let field_inner = match field.network {
                super::NetworkRepr::List => quote! {
                    libquassel::message::NetworkList::from_network_list(&mut std::convert::TryInto::try_into(input.remove(0)).#unwrap)
                },
                super::NetworkRepr::Map => quote! {
                    libquassel::message::NetworkMap::from_network_map(&mut std::convert::TryInto::try_into(input.remove(0)).#unwrap)
                },
                super::NetworkRepr::None => quote! {
                    std::convert::TryInto::try_into(input.remove(0)).#unwrap
                },
            };

            if field.stringlist {
                quote! {
                    #field_name: match input.get_mut(#field_rename).unwrap() {
                        libquassel::primitive::Variant::StringList(input) => #field_inner,
                        _ => panic!("#field_name: wrong variant")
                    },
                }
            } else {
                quote! {
                    #field_name: match input.get_mut(#field_rename).unwrap() {
                        libquassel::primitive::Variant::VariantList(input) => #field_inner,
                        _ => panic!("#field_name: wrong variant")
                    },
                }
            }
        })
        .collect()
}

pub(crate) fn from_vec(type_name: &Ident, fields: &[NetworkField]) -> TokenStream {
    let field = &fields[0];

    let field_rename = match &field.rename {
        Some(name) => name.clone(),
        None => format!("{}", field.ident.as_ref().unwrap()),
    };

    let field_variant = if field.stringlist {
        quote! {libquassel::primitive::StringList}
    } else {
        quote! {libquassel::primitive::VariantList}
    };

    let inner = quote! {
        #type_name::from_network_map(input)
    };

    quote! {
        let marker: #field_variant = std::convert::TryInto::try_into(input.get(#field_rename).unwrap()).unwrap();

        let mut res = Vec::new();
        for _ in 0..marker.len() {
            res.push(#inner);
        }

        return res;
    }
}
