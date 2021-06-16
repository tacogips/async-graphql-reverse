use super::super::parse::{self, *};
use super::dependencies::*;
use super::sorter::sort_by_line_pos;
use super::typ::*;
use super::RenderContext;
use anyhow::Result;
use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::*;
//use syn::*;

pub struct InputFieldsInfo {
    pub members: Vec<TokenStream>,
    pub dependencies: Vec<TokenStream>,
}
impl InputFieldsInfo {
    pub fn new() -> Self {
        Self {
            members: vec![],
            dependencies: vec![],
        }
    }
}

struct InputMember {
    pub member: TokenStream,
    pub dependencies: Vec<TokenStream>,
}

pub fn input_fields_info(
    mut fields: Vec<&parse::InputField>,
    schema: &StructuredSchema,
    context: &RenderContext,
) -> Result<InputFieldsInfo> {
    fields.sort_by(sort_by_line_pos);
    let mut result = InputFieldsInfo::new();
    for field in fields.iter() {
        let InputMember {
            member,
            mut dependencies,
        } = convert_input_field(field, schema, context)?;

        result.members.push(member);

        result.dependencies.append(&mut dependencies);
    }
    Ok(result)
}

fn convert_input_field(
    field: &parse::InputField,
    schema: &StructuredSchema,
    render_context: &RenderContext,
) -> Result<InputMember> {
    let name = format_ident!("{}", field.name_string().to_snake_case());
    let typ = value_type_def_token(&field.typ, &schema)?;
    let member = quote! { pub #name :#typ };

    let dependencies = dependency(&field.typ, schema, render_context)?;

    Ok(InputMember {
        member,
        dependencies,
    })
}
