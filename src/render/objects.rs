use super::super::parse::*;
use super::config::RendererConfig;
use super::dependencies::*;
use super::fields::*;
use super::files::{fmt_file, pathbuf_to_str};
use super::sorter::sort_by_line_pos;
use super::tokens::*;
use super::RenderContext;
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
    objects.sort_by(sort_by_line_pos);

    let mut all_dependencies = HashSet::<String>::new();
    let mut object_defs = Vec::<String>::new();

    let resolver_setting = render_config.resolver_setting();

    for each_obj in objects {
        let (object_token, dependencies) = object_token(
            each_obj,
            &structured_schema,
            render_config,
            &resolver_setting,
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
    resolver_setting: &HashMap<String, HashMap<String, String>>,
) -> Result<(TokenStream, Vec<TokenStream>)> {
    let object_name = format_ident!("{}", object.name);

    let context = RenderContext {
        parent: TypeDef::Object(object),
    };

    let field_resolver = resolver_setting.get(&object.name);

    let FieldsInfo {
        members,
        methods,
        dependencies,
    } = fields_info(
        object.fields.iter().collect(),
        schema,
        render_config,
        &context,
        field_resolver,
    )?;

    let members = separate_by_comma(members);
    let methods = separate_by_space(methods);
    let object_def = quote! {

        #[derive(Debug)]
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
