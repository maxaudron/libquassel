use quote::quote;
use syn::{
    self, bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token, Expr, Result, Token,
};

#[derive(Debug)]
struct Sync {
    name: Expr,
    _token: Token![,],
    _brace_token: token::Bracket,
    fields: Punctuated<Expr, Token![,]>,
}

impl Parse for Sync {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Sync {
            name: input.parse()?,
            _token: input.parse()?,
            _brace_token: bracketed!(content in input),
            fields: content.parse_terminated(Expr::parse, syn::token::Comma)?,
        })
    }
}

pub fn sync(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Sync);
    let name = input.name;
    let values = input.fields.iter();

    let gen = quote! {
        self.send_sync(
            #name,
            vec![#(#values.into()),*],
        )
    };

    gen.into()
}
