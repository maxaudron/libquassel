mod from;
mod network;
mod setters;
mod sync;

#[proc_macro_derive(NetworkList, attributes(network))]
pub fn network_list(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    network::network_list(input)
}

#[proc_macro_derive(NetworkMap, attributes(network))]
pub fn network_map(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    network::network_map(input)
}

#[proc_macro_derive(From, attributes(from))]
pub fn from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from::from(input)
}

#[proc_macro_derive(Setters, attributes(setter))]
pub fn setters(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    setters::setters(input)
}

#[proc_macro]
pub fn sync(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    sync::sync(input)
}

use darling::FromField;

#[derive(Debug, FromField, Clone)]
#[darling(attributes(quassel))]
struct QuasselField {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    name: Option<String>,
}

impl QuasselField {
    pub fn parse(input: &syn::DeriveInput) -> Vec<QuasselField> {
        match &input.data {
            syn::Data::Struct(data) => match &data.fields {
                syn::Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|field| QuasselField::from_field(field).expect("Could not parse quassel field"))
                    .collect(),
                _ => panic!("quassel: not a named field"),
            },
            _ => panic!("quassel: not a Struct"),
        }
    }
}
