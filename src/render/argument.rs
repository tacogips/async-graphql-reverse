use super::super::parse::{self, *};
use super::utils::SnakeCaseWithUnderscores;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::*;

use super::typ::*;
use super::RenderContext;

pub fn argument_def_token(
    argument: &parse::Argument,
    schema: &StructuredSchema,
    name_prefix: &str,
    render_context: &RenderContext,
) -> Result<TokenStream> {
    let name = format_ident!(
        "{}{}",
        name_prefix,
        argument.name_string().to_snake_case_with_underscores()
    );
    let typ = value_type_def_token(&argument.typ, &schema, &render_context)?;

    let result = quote! { #name:#typ };

    Ok(result)
}
