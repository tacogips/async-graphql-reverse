mod argument;
mod config;
mod dependencies;
mod enums;
mod fields;
mod files;
mod input_fields;
mod input_objects;
mod interfaces;
mod keywords;
mod objects;
mod scalars;
mod sorter;
mod tokens;
mod typ;
mod unions;

use super::parse;
use super::parse::*;
use anyhow::{anyhow, Result};
pub use config::*;
use files::{fmt_file, pathbuf_to_str};
use quote::*;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::path::PathBuf;

pub struct RenderContext<'a> {
    parent: parse::TypeDef<'a>,
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

pub fn output(
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

    mod_file(output_dir, log, &structured_schema)?;

    Ok(())
}

fn mod_file(output_dir: &str, info: ModInfo, schema: &StructuredSchema) -> Result<()> {
    let mut output_file = PathBuf::from(output_dir);
    output_file.push("mod.rs");
    let file_path_str = pathbuf_to_str(&output_file);
    let dest_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file.as_path())
        .expect(format!("failed to open file : {}", file_path_str).as_ref());
    let mut dest_file = BufWriter::new(dest_file);

    if info.objects_written {
        dest_file.write(
            quote! { mod objects; use objects::*; }
                .to_string()
                .as_bytes(),
        )?;
    }

    if info.input_objects_written {
        dest_file.write(
            quote! { mod input_objects; use input_objects::*; }
                .to_string()
                .as_bytes(),
        )?;
    }

    if info.union_written {
        dest_file.write(quote! { mod unions; use unions::*; }.to_string().as_bytes())?;
    }

    if info.scalar_written {
        dest_file.write(
            quote! { mod scalars; use scalars::*; }
                .to_string()
                .as_bytes(),
        )?;
    }

    if info.interface_written {
        dest_file.write(
            quote! { mod interfaces; use interfaces::*; }
                .to_string()
                .as_bytes(),
        )?;
    }

    if info.enum_written {
        dest_file.write(quote! { mod enums; use enums::*; }.to_string().as_bytes())?;
    }

    dest_file.write(quote! { use async_graphql::*; }.to_string().as_bytes())?;
    let query_token = schema
        .query_name
        .as_ref()
        .map(|q| {
            let query = format_ident!("{}", q);
            quote! { #query }
        })
        .unwrap_or_else(|| quote! {EmptyQuery});

    let mutation_token = schema
        .mutation_name
        .as_ref()
        .map(|q| {
            let mutation = format_ident!("{}", q);
            quote! { #mutation  }
        })
        .unwrap_or_else(|| quote! {EmptyMutation});

    let schema_token = quote! {
        pub fn schema() -> Schema<#query_token, #mutation_token, EmptySubscription> {
            Schema::new(#query_token{},#mutation_token{}, EmptySubscription)
        }
    };

    dest_file.write(schema_token.to_string().as_bytes())?;
    dest_file.flush()?;

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
