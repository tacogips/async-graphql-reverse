use crate::parse::TypeDef::{
    AsyncGraphqlPreserved, Enum, InputObject, Interface, Object, Primitive, Scalar, Union,
};

use super::super::parse::{self, *};
use super::RenderContext;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::*;

pub fn value_type_def_token(
    type_def: &parse::ValueTypeDef,
    schema: &StructuredSchema,
    render_context: &RenderContext,
) -> Result<TokenStream> {
    let result = match type_def {
        parse::ValueTypeDef::Named(named_value) => {
            let nullable = named_value.is_nullable;
            let type_def = named_value.as_type_def(&schema.definitions)?;
            let type_def = type_def_token(&type_def, &render_context)?;
            if nullable {
                quote! { Option<#type_def > }
            } else {
                quote! { #type_def  }
            }
        }
        parse::ValueTypeDef::List(list_value) => {
            let nullable = list_value.is_nullable;
            let inner_token = value_type_def_token(&list_value.inner, schema, render_context)?;

            if nullable {
                quote! { Option<Vec<#inner_token>>}
            } else {
                quote! { Vec<#inner_token> }
            }
        }
    };
    Ok(result)
}

fn type_def_token(
    type_def: &parse::TypeDef,
    render_context: &RenderContext,
) -> Result<TokenStream> {
    //TODO() impl
    let result = match type_def {
        Primitive(obj) => {
            let name = format_ident!("{}", obj.rust_type());
            quote! { #name }
        }
        obj @ Object(_) | obj @ InputObject(_) => {
            let recursive = if let InputObject(parent) = render_context.parent {
                parent.name == obj.name()
            } else {
                false
            };

            let name: TokenStream = if recursive {
                format!("Box<{}>", obj.name_string()).parse().unwrap()
            } else {
                format!("{}", obj.name_string()).parse().unwrap()
            };
            quote! { #name }
        }
        obj @ Enum(_) | obj @ Scalar(_) | obj @ Union(_) | obj @ Interface(_) => {
            let name = format_ident!("{}", obj.name_string());
            quote! { #name }
        }
        AsyncGraphqlPreserved(obj) => {
            let name = format_ident!("{}", obj);
            quote! { #name }
        }
    };
    Ok(result)
}
