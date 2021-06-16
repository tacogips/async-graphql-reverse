use super::super::parse::{self, *};
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::*;

pub fn value_type_def_token(
    type_def: &parse::ValueTypeDef,
    schema: &StructuredSchema,
) -> Result<TokenStream> {
    let result = match type_def {
        parse::ValueTypeDef::Named(named_value) => {
            let nullable = named_value.is_nullable;
            let type_def = named_value.as_type_def(&schema.definitions)?;
            let type_def = type_def_token(&type_def)?;
            if nullable {
                quote! { Option<#type_def > }
            } else {
                quote! { #type_def  }
            }
        }
        parse::ValueTypeDef::List(list_value) => {
            let inner_token = value_type_def_token(&list_value.inner, schema)?;
            quote! { Vec<#inner_token> }
        }
    };
    Ok(result)
}

fn type_def_token(type_def: &parse::TypeDef) -> Result<TokenStream> {
    let result = match type_def {
        parse::TypeDef::Primitive(primitive) => {
            let name = format_ident!("{}", primitive.rust_type());
            quote! { #name }
        }
        parse::TypeDef::Object(object) => {
            let name = format_ident!("{}", object.name_string());
            quote! { #name }
        }
        parse::TypeDef::Enum(enum_kind) => {
            let name = format_ident!("{}", enum_kind.name_string());
            quote! { #name }
        }
        parse::TypeDef::InputObject(input_object) => {
            let name = format_ident!("{}", input_object.name_string());
            quote! { #name }
        }
        parse::TypeDef::Scalar(scalar) => {
            let name = format_ident!("{}", scalar.name_string());
            quote! { #name }
        }
        parse::TypeDef::Union(union) => {
            let name = format_ident!("{}", union.name_string());
            quote! { #name }
        }
        parse::TypeDef::Interface(interface) => {
            let name = format_ident!("{}", interface.name_string());
            quote! { #name }
        }
    };
    Ok(result)
}