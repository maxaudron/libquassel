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
