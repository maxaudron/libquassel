use quote::quote;
use syn::{self, parse_macro_input};

use darling::{FromDeriveInput, FromField, FromMeta};

mod list;
mod map;
mod maplist;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(network), supports(struct_any))]
/// Derive to and from network methods for quassel objects
pub struct Network {
    /// Representation to choose for the network format
    /// see Repr enum
    #[darling(default)]
    repr: Repr,
}

/// List:
/// Map:
/// Maplist:
#[derive(Debug, Default, Clone, Copy, PartialEq, FromMeta)]
pub enum Repr {
    List,
    #[default]
    Map,
    Maplist,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, FromMeta)]
pub enum NetworkRepr {
    List,
    Map,
    #[default]
    None,
}

#[derive(Debug, FromField)]
#[darling(attributes(network))]
pub struct NetworkField {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    rename: Option<String>,
    #[darling(default, rename = "type")]
    typ: Option<String>,
    /// Variant to encapsulate this field
    /// VariantList (default) or StringList
    #[darling(default)]
    variant: Option<String>,
    /// field is a nested network repr so
    /// use to_network and from_network on it
    #[darling(default)]
    network: NetworkRepr,
    /// Skips this field when parsing from network
    /// representation and uses the default value of the type
    #[darling(default)]
    default: bool,
    /// Skips this field when serializing to network representation
    #[darling(default)]
    skip: bool,

    /// Special reprensentation case for NetworkMap maplist repr
    /// in which case if the field is a String a StringList will be used instead of a VariantList
    /// this cannot be switched with variant as that leads to collisions with the other network representations
    #[darling(default)]
    stringlist: bool,
}

fn parse_fields(input: &syn::DeriveInput) -> Vec<NetworkField> {
    match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| NetworkField::from_field(field).expect("Could not parse field"))
                .collect(),
            _ => panic!("network: not a named field"),
        },
        _ => panic!("network: not a Struct"),
    }
}

pub fn network_map(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let network = Network::from_derive_input(&input).unwrap();

    let mut fields = parse_fields(&input);
    let quassel_fields = super::QuasselField::parse(&input);

    fields.iter_mut().zip(quassel_fields).for_each(|(field, qfield)| {
        if field.rename.is_none() {
            field.rename = qfield.name
        }
    });

    let name = &input.ident;
    let name_str = name.to_string();

    let to_network_map = match network.repr {
        Repr::Maplist => maplist::to(&fields),
        _ => map::to(&fields),
    };

    let from_network_map = match network.repr {
        Repr::Maplist => maplist::from(&fields),
        _ => map::from(&fields),
    };

    let mut gen = quote! {
        impl libquassel::message::signalproxy::NetworkMap for #name {
            type Item = libquassel::primitive::VariantMap;

            fn to_network_map(&self) -> crate::Result<libquassel::primitive::VariantMap> {
                let mut res = libquassel::primitive::VariantMap::new();

                #(#to_network_map)*

                Ok(res)
            }

            fn from_network_map(input: &mut libquassel::primitive::VariantMap) -> crate::Result<Self> {
                log::trace!("converting {} from network object: {:#?}", #name_str, input);
                Ok(Self {
                    #(#from_network_map)*
                })
            }
        }
    };

    let network_map_vec_item = match network.repr {
        Repr::Map => quote! {libquassel::primitive::VariantList},
        Repr::Maplist => quote! {libquassel::primitive::VariantMap},
        Repr::List => quote! {libquassel::primitive::VariantList},
    };

    let to_network_map_vec = match network.repr {
        Repr::Maplist => maplist::to_vec(name, &fields),
        Repr::Map => map::to_vec(name, &fields),
        _ => unimplemented!(),
    };

    let from_network_map_vec = match network.repr {
        Repr::Maplist => maplist::from_vec(name, &fields),
        Repr::Map => map::from_vec(name, &fields),
        _ => unimplemented!(),
    };

    let list_map = quote! {
        impl libquassel::message::signalproxy::NetworkMap for Vec<#name> {
            type Item = #network_map_vec_item;

            fn to_network_map(&self) -> crate::Result<Self::Item> {
                #to_network_map_vec
            }

            fn from_network_map(input: &mut Self::Item) -> crate::Result<Self> {
                #from_network_map_vec
            }
        }
    };

    gen.extend(list_map);

    gen.into()
}

pub fn network_list(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let _network = Network::from_derive_input(&input).unwrap();

    let mut fields = parse_fields(&input);
    let quassel_fields = super::QuasselField::parse(&input);

    fields.iter_mut().zip(quassel_fields).for_each(|(field, qfield)| {
        if field.rename.is_none() {
            field.rename = qfield.name
        }
    });

    let name = &input.ident;
    let name_str = name.to_string();

    let to_network_list = list::to(&fields);

    let gen = quote! {
        impl libquassel::message::signalproxy::NetworkList for #name {
            fn to_network_list(&self) -> crate::Result<libquassel::primitive::VariantList> {
                let mut res = libquassel::primitive::VariantList::new();

                #(#to_network_list)*

                Ok(res)
            }

            fn from_network_list(input: libquassel::primitive::VariantList) -> crate::Result<Self> {
                use crate::message::signalproxy::translation::NetworkMap;
                log::trace!("converting {} from network object: {:#?}", #name_str, input);
                let mut i = input.into_iter();
                let mut map = libquassel::primitive::VariantMap::new();

                while let Some(key) = i.next() {
                    let key: String = key.try_into()?;
                    let value = i.next().ok_or(crate::ProtocolError::MissingField(key.clone()))?;
                    map.insert(key, value);
                }

                Self::from_network_map(&mut map)
            }
        }
    };

    gen.into()
}

fn get_field_type(field: &NetworkField) -> syn::Type {
    if let Some(typ) = &field.typ {
        gen_type(typ)
    } else {
        field.ty.clone()
    }
}

fn get_field_variant_type(field: &NetworkField) -> syn::Type {
    match &field.variant {
        Some(ty) => gen_type(ty),
        None => get_field_type(field),
    }
}

fn gen_type(typ: &str) -> syn::Type {
    syn::Type::from(syn::TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: None,
            segments: {
                let mut res = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::new();

                res.push(syn::PathSegment {
                    ident: syn::Ident::new(typ, proc_macro2::Span::call_site()),
                    arguments: syn::PathArguments::None,
                });

                res
            },
        },
    })
}
