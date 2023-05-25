use super::super::parse::*;
use super::comment::*;
use super::dependencies::*;
use super::fields::*;
use super::files::{fmt_file, pathbuf_to_str};
use super::sorter::sort_by_line_pos_and_name;
use super::tokens::*;
use super::RenderContext;
use crate::config::*;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::*;
use std::collections::{HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn write_objects(
    output_dir: &str,
    structured_schema: &StructuredSchema,
    render_config: &RendererConfig,
) -> Result<bool> {
    let mut output_file = PathBuf::from(output_dir);
    output_file.push("objects.rs");
    let file_path_str = pathbuf_to_str(&output_file);

    if output_file.exists() {
        fs::remove_file(&file_path_str)?;
    }

    let mut objects: Vec<&Object> = structured_schema
        .definitions
        .objects
        .values()
        .into_iter()
        .collect();
    if objects.is_empty() {
        return Ok(false);
    }
    objects.sort_by(sort_by_line_pos_and_name);

    let mut all_dependencies = HashSet::<String>::new();
    let mut object_defs = Vec::<String>::new();

    let custom_member_types = render_config.custom_member_types();
    let resolver_setting = render_config.resolver_setting();
    let additional_resolvers = render_config.additional_resolvers();
    let hidden_fields = render_config.hidden_fields();

    for each_obj in objects {
        let (object_token, dependencies) = object_token(
            each_obj,
            &structured_schema,
            render_config,
            &resolver_setting,
            &custom_member_types,
            &additional_resolvers,
            &hidden_fields,
        )?;

        object_defs.push(object_token.to_string());

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

    let datasource_using: TokenStream = format!("{};", render_config.data_source_using())
        .parse()
        .unwrap();

    dest_file.write(FILE_HEADER_COMMENT.as_bytes())?;
    let header = quote! {
        use async_graphql::*;
        #datasource_using
    };

    dest_file.write(header.to_string().as_bytes())?;
    let dependencies_token = dependency_strs_to_token(all_dependencies);

    dest_file.write(dependencies_token.to_string().as_bytes())?;
    for each_obj_def in object_defs {
        dest_file.write(each_obj_def.as_bytes())?;
    }

    dest_file.flush()?;
    fmt_file(file_path_str)?;
    Ok(true)
}

fn object_token(
    object: &Object,
    schema: &StructuredSchema,
    render_config: &RendererConfig,
    resolver_setting: &HashMap<String, HashMap<String, &ResolverSetting>>,
    custom_member_types: &HashSet<String>,
    additional_resolvers: &HashMap<String, CustomResolvers>,
    hidden_fields: &HashMap<String, HiddenFields>,
) -> Result<(TokenStream, Vec<TokenStream>)> {
    let object_name = format_ident!("{}", object.name);
    let comment = match &object.description {
        Some(desc_token) => to_rust_docs_token(desc_token),
        None => quote! {},
    };

    let context = RenderContext {
        parent: TypeDef::Object(object),
    };

    let field_resolver = resolver_setting.get(&object.name);

    let FieldsInfo {
        mut members,
        mut methods,
        mut dependencies,
    } = fields_info(
        object.fields.iter().collect(),
        schema,
        render_config,
        &context,
        field_resolver,
        &custom_member_types,
    )?;

    if let Some(additional_resolvers) = additional_resolvers.get(&object.name) {
        let mut bodies: Vec<TokenStream> = additional_resolvers
            .bodies
            .iter()
            .map(|e| e.parse::<TokenStream>().unwrap())
            .collect();

        let mut usings: Vec<TokenStream> = additional_resolvers
            .using
            .iter()
            .map(|e| e.parse::<TokenStream>().unwrap())
            .collect();

        methods.append(&mut bodies);
        dependencies.append(&mut usings);
    }

    if let Some(hidden_fields) = hidden_fields.get(&object.name) {
        let mut defs: Vec<TokenStream> = hidden_fields
            .field_defs
            .iter()
            .map(|e| e.parse::<TokenStream>().unwrap())
            .collect();

        let mut usings: Vec<TokenStream> = hidden_fields
            .using
            .iter()
            .map(|e| e.parse::<TokenStream>().unwrap())
            .collect();

        members.append(&mut defs);
        dependencies.append(&mut usings);
    }

    let members = separate_by_comma(members);
    let methods = separate_by_space(methods);
    let object_def = quote! {
        #comment
        #[derive(Debug, Clone)]
        pub struct #object_name{
            #members
        }

        #[Object]
        impl #object_name {
            #methods
        }

    };
    Ok((object_def, dependencies))
}
