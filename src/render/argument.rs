use super::super::parse::{self, *};
use anyhow::Result;
use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::*;

use super::typ::*;

pub fn argument_def_token(
    argument: &parse::Argument,
    schema: &StructuredSchema,
    name_prefix: &str,
) -> Result<TokenStream> {
    let name = format_ident!("{}{}", name_prefix, argument.name_string().to_snake_case());
    let typ = value_type_def_token(&argument.typ, &schema)?;

    let result = quote! { #name:#typ };

    Ok(result)
}
