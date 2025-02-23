use darling::FromField;
use proc_macro2::Span;
use quote::quote;
use syn::{self, parse_macro_input};

#[derive(Debug, FromField)]
#[darling(attributes(setter))]
pub struct SetterField {
    ident: Option<syn::Ident>,

    #[darling(default)]
    skip: bool,

    #[darling(default)]
    /// name for the variable
    /// mainly to show up in the docs
    /// by default using the last word of the name
    var_name: Option<syn::Ident>,

    #[darling(default)]
    /// Override the name of the function if the desired name does not match the variable name.
    name: Option<syn::Ident>,
}

impl SetterField {
    pub fn parse(input: &syn::DeriveInput) -> Vec<SetterField> {
        match &input.data {
            syn::Data::Struct(data) => match &data.fields {
                syn::Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|field| SetterField::from_field(field).expect("Could not parse setter field"))
                    .collect(),
                _ => panic!("setter: not a named field"),
            },
            _ => panic!("setter: not a Struct"),
        }
    }
}

pub fn setters(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let setter_fields = SetterField::parse(&input);
    let quassel_fields = super::QuasselField::parse(&input);

    let generated: Vec<proc_macro2::TokenStream> = setter_fields
        .iter()
        .zip(quassel_fields)
        .filter(|(setter_field, _)| !setter_field.skip)
        .map(|(setter_field, quassel_field)| {
            let raw_name = if let Some(name) = &quassel_field.name {
                name.clone()
            } else {
                setter_field.ident.as_ref().unwrap().to_string()
            };

            let name: String = {
                let mut name = raw_name.chars();

                format!(
                    "set{}{}",
                    name.next().unwrap().to_ascii_uppercase(),
                    name.as_str()
                )
            };

            let var_name = syn::Ident::new(
                &setter_field.var_name.as_ref().map(|v| v.to_string()).unwrap_or({
                    let mut res = String::new();

                    for c in raw_name.chars().rev() {
                        if c <= 'Z' && c >= 'A' {
                            res.push(c.to_ascii_lowercase());
                            break;
                        } else {
                            res.push(c);
                        }
                    }

                    res.chars().rev().collect()
                }),
                Span::call_site(),
            );

            let ident = quassel_field
                .ident
                .as_ref()
                .expect("failed to get quassel field ident");
            let ty = &quassel_field.ty;

            let fn_ident = if let Some(name) = setter_field.name.clone() {
                name
            } else {
                quassel_field
                    .ident
                    .as_ref()
                    .expect("failed to get quassel field ident")
                    .clone()
            };

            let fn_name = syn::Ident::new(&format!("set_{}", fn_ident), Span::call_site());

            quote! {
                pub fn #fn_name(&mut self, #var_name: #ty) {
                    #[cfg(feature = "server")]
                    self.send_sync(#name, vec![#var_name.clone().into()]);

                    self.#ident = #var_name;
                }
            }
        })
        .collect();

    let struct_name = &input.ident;
    let gen = quote! {
        impl #struct_name {
            #(#generated)*
        }
    };

    return gen.into();
}
