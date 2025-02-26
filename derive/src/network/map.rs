use proc_macro2::{Ident, TokenStream};
use quote::quote;

use super::{get_field_variant_type, NetworkField};

pub(crate) fn to(fields: &Vec<NetworkField>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            if !field.skip {
                let field_rename = match &field.rename {
                    Some(name) => name.clone(),
                    None => format!("{}", field.ident.as_ref().unwrap()).into(),
                };

                let field_name = field.ident.as_ref().unwrap();
                let field_variant_type = get_field_variant_type(&field);

                let field_inner = match field.network {
                    crate::network::NetworkRepr::List => quote! {
                        libquassel::message::NetworkList::to_network_list(&self.#field_name).into()
                    },
                    crate::network::NetworkRepr::Map => quote! {
                        libquassel::message::NetworkMap::to_network_map(&self.#field_name).into()
                    },
                    crate::network::NetworkRepr::None => quote! {
                        self.#field_name.clone().into()
                    },
                };

                quote! {
                    res.insert(#field_rename.to_string(),
                        libquassel::primitive::Variant::#field_variant_type(#field_inner));
                }
            } else {
                quote! {}
            }
        })
        .collect()
}

pub(crate) fn from(fields: &Vec<NetworkField>) -> Vec<TokenStream> {
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
                None => format!("{}", field.ident.as_ref().unwrap()).into(),
            };

            match field.network {
                super::NetworkRepr::List => quote! {
                    #field_name: libquassel::message::NetworkList::from_network_list(
                        &mut std::convert::TryInto::try_into(input.remove(#field_rename).unwrap()).#unwrap),
                },
                super::NetworkRepr::Map => quote! {
                    #field_name: libquassel::message::NetworkMap::from_network_map(
                        &mut std::convert::TryInto::try_into(input.remove(#field_rename).unwrap()).#unwrap),
                },
                super::NetworkRepr::None => quote! {
                    #field_name: std::convert::TryInto::try_into(input.remove(#field_rename).unwrap()).#unwrap,
                },
            }
        })
        .collect()
}

pub(crate) fn to_vec(_type_name: &Ident, _fields: &Vec<NetworkField>) -> TokenStream {
    quote! {
        self.iter().map(|item| {
            item.to_network_map().into()
        }).collect()
    }
}

pub(crate) fn from_vec(type_name: &Ident, _fields: &Vec<NetworkField>) -> TokenStream {
    quote! {
        input.iter().map(
            |item| #type_name::from_network_map(
                &mut std::convert::TryInto::try_into(item).unwrap()
            )).collect()
    }
}
