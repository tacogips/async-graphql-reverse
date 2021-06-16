use super::super::parse::{self, *};
use super::tokens::*;
use super::RenderContext;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::*;
use std::collections::HashSet;

pub fn dependency(
    type_def: &parse::ValueTypeDef,
    schema: &StructuredSchema,
    context: &RenderContext,
) -> Result<Vec<TokenStream>> {
    let source_type = parse::source_type_def(&type_def, schema)?;
    let result = match source_type {
        parse::TypeDef::Primitive(_) => {
            return Ok(vec![]);
        }
        parse::TypeDef::Object(object) => {
            if context.parent.is_object() {
                return Ok(vec![]);
            }
            let name = format_ident!("{}", object.name_string());
            quote! { use super::objects::#name }
        }
        parse::TypeDef::Enum(enum_kind) => {
            if context.parent.is_enum() {
                return Ok(vec![]);
            }
            let name = format_ident!("{}", enum_kind.name_string());
            quote! { use super::enums::#name }
        }
        parse::TypeDef::InputObject(input_object) => {
            if context.parent.is_input_object() {
                return Ok(vec![]);
            }
            let name = format_ident!("{}", input_object.name_string());
            quote! { use super::input_objects::#name }
        }
        parse::TypeDef::Scalar(scalar) => {
            if context.parent.is_scalar() {
                return Ok(vec![]);
            }
            let name = format_ident!("{}", scalar.name_string());
            quote! { use super::scalars::#name }
        }
        parse::TypeDef::Union(union) => {
            if context.parent.is_union() {
                return Ok(vec![]);
            }
            let name = format_ident!("{}", union.name_string());
            quote! { use super::unions::#name }
        }
        parse::TypeDef::Interface(interface) => {
            if context.parent.is_interface() {
                return Ok(vec![]);
            }
            let name = format_ident!("{}", interface.name_string());
            quote! { use super::interfaces::#name }
        }
    };
    Ok(vec![result])
}

pub fn dependency_strs_to_token(dependencies: HashSet<String>) -> TokenStream {
    merge_with_trailing_semicomman(
        dependencies
            .into_iter()
            .map(|dep| {
                let dep: TokenStream = dep.parse().unwrap();
                quote! {#dep}
            })
            .collect(),
    )
}
