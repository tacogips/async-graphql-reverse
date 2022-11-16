use super::super::parse::*;
use super::comment::*;
use super::dependencies::*;
use super::files::{fmt_file, pathbuf_to_str};
use super::sorter::sort_by_line_pos;
use super::tokens::*;
use super::typ::*;
use super::RenderContext;
use crate::config::RendererConfig;
use anyhow::Result;
use heck::{CamelCase, SnakeCase};
use proc_macro2::TokenStream;
use quote::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn write_interfaces(
    output_dir: &str,
    structured_schema: &StructuredSchema,
    render_config: &RendererConfig,
) -> Result<bool> {
    let mut output_file = PathBuf::from(output_dir);
    output_file.push("interfaces.rs");
    let file_path_str = pathbuf_to_str(&output_file);

    if output_file.exists() {
        fs::remove_file(&file_path_str)?;
    }

    let mut interfaces: Vec<&Interface> = structured_schema
        .definitions
        .interfaces
        .values()
        .into_iter()
        .collect();
    if interfaces.is_empty() {
        return Ok(false);
    }
    interfaces.sort_by(sort_by_line_pos);

    let mut all_dependencies = HashSet::<String>::new();
    let mut interface_defs = Vec::<String>::new();

    let interface_and_impl_types = find_implment_types_by_interface_type(&structured_schema);

    for each_obj in interfaces {
        let (interface_token, dependencies) = interface_token(
            each_obj,
            &structured_schema,
            render_config,
            &interface_and_impl_types,
        )?;

        interface_defs.push(interface_token.to_string());

        for each_dep in dependencies.into_iter() {
            all_dependencies.insert(each_dep.to_string());
        }
    }

    let dest_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file.as_path())
        .expect(format!("failed to open file : {}", file_path_str).as_ref());
    let mut dest_file = BufWriter::new(dest_file);

    dest_file.write(FILE_HEADER_COMMENT.as_bytes())?;
    let header = quote! {
        use async_graphql::*;
    };

    dest_file.write(header.to_string().as_bytes())?;
    let dependencies_token = dependency_strs_to_token(all_dependencies);

    dest_file.write(dependencies_token.to_string().as_bytes())?;
    for each_obj_def in interface_defs {
        dest_file.write(each_obj_def.as_bytes())?;
    }

    dest_file.flush()?;
    fmt_file(file_path_str)?;
    Ok(true)
}

fn interface_token(
    interface: &Interface,
    schema: &StructuredSchema,
    _render_config: &RendererConfig,
    interface_type_and_impl_types: &HashMap<String, Vec<String>>,
) -> Result<(TokenStream, Vec<TokenStream>)> {
    let interface_name = format_ident!("{}", interface.name);

    let context = RenderContext {
        parent: TypeDef::Interface(interface),
    };

    let mut interface_field_tokens = Vec::<TokenStream>::new();
    let mut all_dependency_tokens = Vec::<TokenStream>::new();

    let render_context = RenderContext {
        parent: TypeDef::Interface(interface),
    };

    for interface_field in interface.fields.iter() {
        let field_name = &interface_field.name.to_snake_case();
        let field_type = value_type_def_token(&interface_field.typ, &schema, &render_context)?
            .to_string()
            .replace(" ", "");

        let field_token = quote! {field(name = #field_name, type = #field_type )};
        interface_field_tokens.push(field_token);

        let mut dependencies = dependency(&interface_field.typ, schema, &context)?;
        all_dependency_tokens.append(&mut dependencies);
    }

    let mut interface_memer_tokens = Vec::<TokenStream>::new();
    if let Some(impl_types) = interface_type_and_impl_types.get(&interface.name) {
        let mut impl_types = impl_types.clone();
        impl_types.sort();
        for member in impl_types {
            let mut interface_member = convert_interface_member(&member, schema, &context)?;
            interface_memer_tokens.push(interface_member.member);

            all_dependency_tokens.append(&mut interface_member.dependencies);
        }
    }

    let interface_fields_token = separate_by_comma(interface_field_tokens);
    let interface_memer_tokens = separate_by_comma(interface_memer_tokens);
    let interface_def = quote! {

        #[derive(Interface)]
        #[graphql(#interface_fields_token)]
        #[derive(Debug, Clone)]
        pub enum #interface_name{
            #interface_memer_tokens
        }

    };
    Ok((interface_def, all_dependency_tokens))
}

struct InterfaceMember {
    pub member: TokenStream,
    pub dependencies: Vec<TokenStream>,
}

fn find_implment_types_by_interface_type(
    structured_schema: &StructuredSchema,
) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::<String, Vec<String>>::new();
    for each_obj in structured_schema.definitions.objects.values() {
        for interface_type in each_obj.impl_interface_name.iter() {
            let impl_types = result.entry(interface_type.to_string()).or_insert(vec![]);
            impl_types.push(each_obj.name.to_string());
        }
    }
    result
}

fn convert_interface_member(
    member: &str,
    schema: &StructuredSchema,
    render_context: &RenderContext,
) -> Result<InterfaceMember> {
    let member_type_name = format_ident!("{}", member);
    let member_enum_name = format_ident!("{}", member.to_camel_case());

    //TODO(tacogips) this conversion of interface member type to ValueTypeDef might be a bit hack-y?
    let member_type = ValueTypeDef::Named(NamedValue {
        value_type_name: member.to_string(),
        is_nullable: false,
    });
    let dependencies = dependency(&member_type, schema, render_context)?;

    let member = quote! { #member_enum_name (#member_type_name) };

    Ok(InterfaceMember {
        member,
        dependencies,
    })
}
