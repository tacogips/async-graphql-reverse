mod argument;
mod comment;
mod datasource;
mod dependencies;
mod enums;
mod fields;
mod files;
mod input_fields;
mod input_objects;
mod interfaces;
mod keywords;
mod linter;
mod objects;
mod scalars;
mod sorter;
mod tokens;
mod typ;
mod unions;

use super::parse;
use super::parse::*;
use crate::config::RendererConfig;
use anyhow::{anyhow, Result};
use comment::*;
use files::{fmt_file, pathbuf_to_str};
use linter::*;
use quote::*;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

pub struct RenderContext<'a> {
    pub parent: parse::TypeDef<'a>,
}

impl<'a> RenderContext<'a> {
    pub fn parent_name(&self) -> String {
        match self.parent {
            parse::TypeDef::Object(obj) => format!("{}", obj.name_string()),
            parse::TypeDef::Enum(obj) => format!("{}", obj.name_string()),
            parse::TypeDef::InputObject(obj) => format!("{}", obj.name_string()),
            parse::TypeDef::Union(obj) => format!("{}", obj.name_string()),
            parse::TypeDef::Interface(obj) => format!("{}", obj.name_string()),
            _ => panic!("invalid parent : {:?}", self.parent),
        }
    }
}

struct ModInfo {
    objects_written: bool,
    input_objects_written: bool,
    union_written: bool,
    scalar_written: bool,
    interface_written: bool,
    enum_written: bool,
}

pub fn output_datasource(
    output_dir: &str,
    structured_schema: StructuredSchema,
    config: &RendererConfig,
) -> Result<()> {
    setup_output_dir(output_dir)?;
    datasource_mod_file(output_dir, &structured_schema, &config)?;
    Ok(())
}

pub fn output_schema(
    output_dir: &str,
    structured_schema: StructuredSchema,
    config: RendererConfig,
) -> Result<()> {
    setup_output_dir(output_dir)?;

    let objects_written = objects::write_objects(output_dir, &structured_schema, &config)?;

    let input_objects_written = input_objects::write_input_objects(output_dir, &structured_schema)?;
    let union_written = unions::write_unions(output_dir, &structured_schema)?;
    let scalar_written = scalars::write_scalars(output_dir, &structured_schema)?;
    let interface_written = interfaces::write_interfaces(output_dir, &structured_schema, &config)?;
    let enum_written = enums::write_enums(output_dir, &structured_schema)?;

    let log = ModInfo {
        objects_written,
        input_objects_written,
        union_written,
        scalar_written,
        interface_written,
        enum_written,
    };

    schema_mod_file(output_dir, log, &structured_schema)?;

    Ok(())
}

fn schema_mod_file(output_dir: &str, info: ModInfo, schema: &StructuredSchema) -> Result<()> {
    let mut output_file = PathBuf::from(output_dir);
    output_file.push("mod.rs");
    if output_file.exists() {
        fs::remove_file(&output_file)?;
    }
    let file_path_str = pathbuf_to_str(&output_file);

    let dest_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file.as_path())
        .expect(format!("failed to open file : {}", file_path_str).as_ref());
    let mut dest_file = BufWriter::new(dest_file);

    dest_file.write(SUPPRESS_LINT.as_bytes())?;
    dest_file.write(FILE_HEADER_COMMENT.as_bytes())?;
    if info.objects_written {
        dest_file.write(
            quote! { mod objects; pub use objects::*; }
                .to_string()
                .as_bytes(),
        )?;
    }

    if info.input_objects_written {
        dest_file.write(
            quote! { mod input_objects; pub use input_objects::*; }
                .to_string()
                .as_bytes(),
        )?;
    }

    if info.union_written {
        dest_file.write(
            quote! { mod unions; pub use unions::*; }
                .to_string()
                .as_bytes(),
        )?;
    }

    if info.scalar_written {
        dest_file.write(
            quote! { mod scalars; pub use scalars::*; }
                .to_string()
                .as_bytes(),
        )?;
    }

    if info.interface_written {
        dest_file.write(
            quote! { mod interfaces; pub use interfaces::*; }
                .to_string()
                .as_bytes(),
        )?;
    }

    if info.enum_written {
        dest_file.write(
            quote! { mod enums; pub use enums::*; }
                .to_string()
                .as_bytes(),
        )?;
    }

    match schema.query_name.as_ref().map(|q| {
        let query = format_ident!("{}", q);
        quote! { #query }
    }) {
        Some(query_token) => {
            dest_file.write(quote! { use async_graphql::*; }.to_string().as_bytes())?;
            let mutation_token = schema
                .mutation_name
                .as_ref()
                .map(|q| {
                    let mutation = format_ident!("{}", q);
                    quote! { #mutation  }
                })
                .unwrap_or_else(|| quote! {EmptyMutation});

            let schema_token = quote! {
                pub fn schema_builder() -> SchemaBuilder<#query_token, #mutation_token, EmptySubscription> {
                    Schema::build(#query_token{},#mutation_token{}, EmptySubscription)
                }
            };

            dest_file.write(schema_token.to_string().as_bytes())?;
            dest_file.flush()?;
        }
        None => {
            let schema_token = r#"
                // Skip building schema_builder() due to no query defined.
                // // example schema_builder()
                // pub fn schema_builder() -> SchemaBuilder<YourQueryType, EmptyMutation, EmptySubscription> {
                //     Schema::build(YourQueryType, EmptyMutation, EmptySubscription)
                // }
            "#;

            dest_file.write(schema_token.as_bytes())?;
            dest_file.flush()?;
        }
    }

    fmt_file(file_path_str)?;

    Ok(())
}

pub fn setup_output_dir(output_dir: &str) -> Result<()> {
    let output_path = Path::new(output_dir);
    if output_path.exists() {
        let output_metadata = fs::metadata(output_dir)?;
        if !output_metadata.is_dir() {
            return Err(anyhow!("output path {} is not dir.", output_dir));
        }
    } else {
        fs::create_dir_all(output_dir)?;
    }
    Ok(())
}

fn datasource_mod_file(
    output_dir: &str,
    schema: &StructuredSchema,
    render_config: &RendererConfig,
) -> Result<()> {
    let mut output_file = PathBuf::from(output_dir);
    output_file.push("mod.rs");
    if output_file.exists() {
        fs::remove_file(&output_file)?;
    }

    let file_path_str = pathbuf_to_str(&output_file);
    let dest_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file.as_path())
        .expect(format!("failed to open file : {}", file_path_str).as_ref());
    let mut dest_file = BufWriter::new(dest_file);

    let header = quote! {
         use async_graphql::*;
    };
    dest_file.write(header.to_string().as_bytes())?;

    let methods = datasource::empty_datasource_methods(schema, render_config)?;
    let methods = tokens::separate_by_space(methods);

    let datasource = quote! {
        pub struct DataSource{}

        impl  DataSource{
            #methods
        }
    };

    dest_file.write(datasource.to_string().as_bytes())?;

    dest_file.flush()?;

    fmt_file(file_path_str)?;

    Ok(())
}
