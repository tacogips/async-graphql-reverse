use super::super::parse::*;
use super::comment::*;
use super::files::{fmt_file, pathbuf_to_str};
use super::sorter::sort_by_line_pos_and_name;
use super::tokens::*;
use crate::config::{EnumSetting, EnumValueSetting, RendererConfig};
use anyhow::Result;
use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::*;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn write_enums(
    output_dir: &str,
    structured_schema: &StructuredSchema,
    config: &RendererConfig,
) -> Result<bool> {
    let mut output_file = PathBuf::from(output_dir);
    output_file.push("enums.rs");
    let file_path_str = pathbuf_to_str(&output_file);

    if output_file.exists() {
        fs::remove_file(&file_path_str)?;
    }

    let mut enums: Vec<&Enum> = structured_schema
        .definitions
        .enums
        .values()
        .into_iter()
        .collect();
    if enums.is_empty() {
        return Ok(false);
    }
    enums.sort_by(sort_by_line_pos_and_name);

    let mut enum_defs = Vec::<String>::new();

    let enum_settings = config.enum_settings();

    for each_enum in enums {
        let enum_token = enum_token(each_enum, &structured_schema, config, &enum_settings)?;
        enum_defs.push(enum_token.to_string());
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

    for each_obj_def in enum_defs {
        dest_file.write(each_obj_def.as_bytes())?;
    }

    dest_file.flush()?;
    fmt_file(file_path_str)?;
    Ok(true)
}

fn enum_token(
    enm: &Enum,
    _schema: &StructuredSchema,
    config: &RendererConfig,
    enum_settings: &HashMap<String, EnumSetting>,
) -> Result<TokenStream> {
    let enum_name = enm.name.to_upper_camel_case();
    let mut graphql_derive = quote! {};

    // TODO(tacogips) using there_is_specific_rename_item is naive implementation. make this concise with macro or something
    let mut there_is_specific_rename_item = false;
    let mut enum_value_settings = HashMap::<String, &EnumValueSetting>::default();
    if let Some(specific_enum_setting) = enum_settings.get(&enum_name) {
        if let Some(specifig_rename_items) = &specific_enum_setting.rename_items {
            there_is_specific_rename_item = true;
            graphql_derive = quote! {
                #[graphql(rename_items = #specifig_rename_items)]
            }
        }

        if let Some(specific_enum_setting_value) = specific_enum_setting.value.as_ref() {
            enum_value_settings = specific_enum_setting_value
                .iter()
                .map(|each| (each.value.to_string(), each))
                .collect();
        }
    }

    let enums_members: Vec<TokenStream> = enm
        .values
        .iter()
        .map(|each_enum_value| {
            //each_enum.value_name.parse::<TokenStream>().unwrap()}
            let enum_value_name = each_enum_value.value_name.to_upper_camel_case();
            let each_enum = format_ident!("{}", enum_value_name);

            let enum_attribute = match enum_value_settings.get(&enum_value_name) {
                Some(each_enum_setting) => match &each_enum_setting.rename {
                    Some(rename) => {
                        quote! {
                            #[graphql(name = #rename)]
                        }
                    }
                    None => quote! {},
                },
                None => quote! {},
            };

            quote! {
                #enum_attribute
                #each_enum
            }
        })
        .collect();

    if !there_is_specific_rename_item {
        if let Some(enum_rename_items) = config.enum_rename_items.as_ref() {
            graphql_derive = quote! {
                    #[graphql(rename_items = #enum_rename_items)]
            }
        }
    }

    let enum_name = format_ident!("{}", enum_name);
    let enum_members = separate_by_comma(enums_members);

    let enum_def = quote! {

        #[derive(Enum, Copy, Clone, Debug, Eq, PartialEq)]
        #graphql_derive
        pub enum #enum_name{
            #enum_members
        }


    };
    Ok(enum_def)
}
