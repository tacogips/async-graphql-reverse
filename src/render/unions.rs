use super::super::parse::*;
use super::comment::*;
use super::dependencies::*;
use super::files::{fmt_file, pathbuf_to_str};
use super::sorter::sort_by_line_pos_and_name;
use super::tokens::*;
use super::RenderContext;
use anyhow::Result;
use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::*;
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn write_unions(output_dir: &str, structured_schema: &StructuredSchema) -> Result<bool> {
    let mut output_file = PathBuf::from(output_dir);
    output_file.push("unions.rs");
    let file_path_str = pathbuf_to_str(&output_file);

    if output_file.exists() {
        fs::remove_file(&file_path_str)?;
    }

    let mut unions: Vec<&Union> = structured_schema
        .definitions
        .unions
        .values()
        .into_iter()
        .collect();
    if unions.is_empty() {
        return Ok(false);
    }
    unions.sort_by(sort_by_line_pos_and_name);

    let mut all_dependencies = HashSet::<String>::new();
    let mut union_defs = Vec::<String>::new();

    for each_union in unions {
        let (union_token, dependencies) = union_token(each_union, &structured_schema)?;

        union_defs.push(union_token.to_string());

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
    for each_union_def in union_defs {
        dest_file.write(each_union_def.as_bytes())?;
    }

    dest_file.flush()?;
    fmt_file(file_path_str)?;
    Ok(true)
}

fn union_token(
    union: &Union,
    schema: &StructuredSchema,
) -> Result<(TokenStream, Vec<TokenStream>)> {
    let union_name = format_ident!("{}", union.name);

    let context = RenderContext {
        parent: TypeDef::Union(union),
    };

    let UnionFieldsInfo {
        members,
        dependencies,
    } = union_fields_info(union.type_names.iter().collect(), schema, &context)?;

    let members = separate_by_comma(members);
    let union_def = quote! {

        #[derive(Union, Debug, Clone)]
        pub enum #union_name {
            #members
        }


    };
    Ok((union_def, dependencies))
}

pub fn union_fields_info(
    mut members: Vec<&String>,
    schema: &StructuredSchema,
    context: &RenderContext,
) -> Result<UnionFieldsInfo> {
    members.sort();
    let mut result = UnionFieldsInfo::new();
    for member in members.iter() {
        let UnionMember {
            member,
            mut dependencies,
        } = convert_union_member(member, schema, context)?;

        result.members.push(member);

        result.dependencies.append(&mut dependencies);
    }
    Ok(result)
}

pub struct UnionFieldsInfo {
    pub members: Vec<TokenStream>,
    pub dependencies: Vec<TokenStream>,
}

impl UnionFieldsInfo {
    pub fn new() -> Self {
        Self {
            members: vec![],
            dependencies: vec![],
        }
    }
}

struct UnionMember {
    pub member: TokenStream,
    pub dependencies: Vec<TokenStream>,
}

fn convert_union_member(
    member: &str,
    schema: &StructuredSchema,
    render_context: &RenderContext,
) -> Result<UnionMember> {
    let member_type_name = format_ident!("{}", member);
    let member_enum_name = format_ident!("{}", member.to_upper_camel_case());

    //TODO(tacogips) this conversion of union member type to ValueTypeDef might be a bit hack-y?
    let member_type = ValueTypeDef::Named(NamedValue {
        value_type_name: member.to_string(),
        is_nullable: false,
    });
    let dependencies = dependency(&member_type, schema, render_context)?;

    let member = quote! { #member_enum_name (#member_type_name) };

    Ok(UnionMember {
        member,
        dependencies,
    })
}
