use proc_macro2::TokenStream;
use quote::quote;

use super::{get_field_variant_type, NetworkField};

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
                let field_type = get_field_variant_type(field);

                let field_inner = match field.network {
                    crate::network::NetworkRepr::List => {
                        quote! { libquassel::message::NetworkList::to_network_list(&self.#field_name)?.into() }
                    }
                    crate::network::NetworkRepr::Map => {
                        quote! { libquassel::message::NetworkMap::to_network_map(&self.#field_name)?.into() }
                    }
                    crate::network::NetworkRepr::None => quote! { self.#field_name.clone().into() },
                };

                quote! {
                    res.push(libquassel::primitive::Variant::ByteArray(#field_rename.to_string()));
                    res.push(libquassel::primitive::Variant::#field_type(#field_inner));
                }
            } else {
                quote! {}
            }
        })
        .collect()
}

pub(crate) fn from(fields: &[NetworkField]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();

            let field_rename = match &field.rename {
                Some(name) => name.clone(),
                None => format!("{}", field.ident.as_ref().unwrap()),
            };

            let field_variant_type = get_field_variant_type(field);

            let extract_inner = if field.default {
                quote! {
                    let mut i = input.iter();
                    match i.position(|x| *x == libquassel::primitive::Variant::ByteArray(String::from(#field_rename))) {
                        Some(_) => {
                            match i.next().ok_or_else(|| crate::ProtocolError::MissingField(#field_rename.to_string()))? {
                                libquassel::primitive::Variant::#field_variant_type(var) => var
                                    .clone()
                                    .try_into()
                                    .unwrap_or(Default::default()),
                                _ => Default::default(),
                            }
                        }
                        None => Default::default(),
                    }
                }
            } else {
                quote! {
                    let mut i = input.iter();
                    i.position(|x| *x == libquassel::primitive::Variant::ByteArray(String::from(#field_rename)))
                        .expect(format!("failed to get field {}", #field_rename).as_str());

                    match i.next().expect("failed to get next field") {
                        libquassel::primitive::Variant::#field_variant_type(var) => var.clone().try_into().unwrap(),
                        _ => panic!("network::list::from: wrong variant type"),
                    }
                }
            };

            match field.network {
                super::NetworkRepr::List => quote! {
                        #field_name: libquassel::message::NetworkList::from_network_list(&mut {
                            #extract_inner
                        })?,
                    },
                super::NetworkRepr::Map => quote! {
                        #field_name: libquassel::message::NetworkMap::from_network_map(&mut {
                            #extract_inner
                        })?,
                    },
                super::NetworkRepr::None => quote! {
                    #field_name: {
                        #extract_inner
                    },
                },
            }
        })
        .collect()
}
