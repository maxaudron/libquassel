use quote::quote;
use syn::parse_macro_input;

use darling::{FromDeriveInput, FromVariant};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(from), supports(enum_any))]
struct Enum {
    ident: syn::Ident,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(from))]
struct EnumField {
    ident: syn::Ident,
    fields: darling::ast::Fields<syn::Type>,

    #[darling(default)]
    ignore: bool,
}

pub fn from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    // println!("{:#?}", input);

    let network = Enum::from_derive_input(&input).unwrap();
    // println!("{:#?}", network);

    let enum_name = network.ident;

    let fields: Vec<EnumField> = match &input.data {
        syn::Data::Enum(data) => data
            .variants
            .iter()
            .map(|field| EnumField::from_variant(field).expect("Could not parse field"))
            .collect(),
        _ => panic!("from: not an enum"),
    };

    let derives = fields
        .iter()
        .filter(|field| !field.fields.fields.is_empty() && !field.ignore)
        .map(|field| {
            let variant = &field.ident;
            let inner_type = &field.fields.fields[0];

            quote! {
                impl From<#inner_type> for #enum_name {
                    fn from(input: #inner_type) -> Self {
                        Self::#variant(input)
                    }
                }

                impl std::convert::TryFrom<#enum_name> for #inner_type {
                    type Error = crate::error::ProtocolError;

                    fn try_from(input: #enum_name) -> Result<Self, Self::Error> {
                        match input {
                            #enum_name::#variant(input) => Ok(input),
                            _ => Err(crate::error::ProtocolError::WrongVariant),
                        }
                    }
                }

                impl std::convert::TryFrom<&#enum_name> for #inner_type {
                    type Error = crate::error::ProtocolError;

                    fn try_from(input: &#enum_name) -> Result<Self, Self::Error> {
                        match input {
                            #enum_name::#variant(input) => Ok(input.clone()),
                            _ => Err(crate::error::ProtocolError::WrongVariant),
                        }
                    }
                }
            }
        });

    // println!("{:#?}", fields);

    let gen = quote! {
        #(#derives)*
    };

    // println!("{}", gen);

    gen.into()
}
