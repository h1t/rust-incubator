use proc_macro::TokenStream;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::Expr;
use syn::Result;
use syn::Token;

struct Pair {
    key: Expr,
    _arrow: Token![=>],
    value: Expr,
}

impl Parse for Pair {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            key: input.parse()?,
            _arrow: input.parse()?,
            value: input.parse()?,
        })
    }
}

struct PairList {
    pairs: Punctuated<Pair, Token![,]>,
}

impl Parse for PairList {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            pairs: input.parse_terminated(Pair::parse, Token![,])?,
        })
    }
}

#[proc_macro]
pub fn btreemap(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as PairList);
    let pairs = input.pairs.into_iter().map(|Pair { key, value, .. }| {
        quote::quote! { (#key, #value) }
    });

    quote::quote! { std::collections::BTreeMap::from([#(#pairs),*]) }.into()
}
