use proc_macro2::TokenStream;
use quote::*;

pub fn separate_by_space(tokens: Vec<TokenStream>) -> TokenStream {
    separate_tokens_by(tokens, " ")
}

pub fn separate_by_comma(tokens: Vec<TokenStream>) -> TokenStream {
    separate_tokens_by(tokens, ",")
}

fn separate_tokens_by(mut tokens: Vec<TokenStream>, by: &'static str) -> TokenStream {
    if tokens.is_empty() {
        quote! {}
    } else {
        let by: TokenStream = by.parse().unwrap();
        let first = tokens.remove(0);
        let result = tokens
            .iter()
            .fold(quote! { #first }, |acc, each| quote! {#acc #by #each});
        result
    }
}

pub fn merge_with_trailing_semicomman(mut tokens: Vec<TokenStream>) -> TokenStream {
    if tokens.is_empty() {
        quote! {}
    } else {
        let by: TokenStream = ";".parse().unwrap();
        let first = tokens.remove(0);
        let result = tokens
            .iter()
            .fold(quote! {#first #by}, |acc, each| quote! {#acc #each #by });
        result
    }
}
