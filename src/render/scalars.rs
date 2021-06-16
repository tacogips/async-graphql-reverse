use super::super::parse::*;
use super::files::{fmt_file, pathbuf_to_str};
use super::sorter::sort_by_line_pos;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::*;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn write_scalars(output_dir: &str, structured_schema: &StructuredSchema) -> Result<bool> {
    let mut output_file = PathBuf::from(output_dir);
    output_file.push("scalars.rs");
    let file_path_str = pathbuf_to_str(&output_file);

    if output_file.exists() {
        fs::remove_file(&file_path_str)?;
    }

    let mut scalars: Vec<&Scalar> = structured_schema
        .definitions
        .scalars
        .values()
        .into_iter()
        .collect();
    if scalars.is_empty() {
        return Ok(false);
    }
    scalars.sort_by(sort_by_line_pos);

    let mut scalar_defs = Vec::<String>::new();

    for each_scalar in scalars {
        let scalar_token = scalar_token(each_scalar, &structured_schema)?;
        scalar_defs.push(scalar_token.to_string());
    }

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

    for each_obj_def in scalar_defs {
        dest_file.write(each_obj_def.as_bytes())?;
    }

    dest_file.flush()?;
    fmt_file(file_path_str)?;
    Ok(true)
}

fn scalar_token(scalar: &Scalar, schema: &StructuredSchema) -> Result<TokenStream> {
    let scalar_name = format_ident!("{}", scalar.name);

    let scalar_def = quote! {

    #[derive(Debug, Clone)]
    pub struct #scalar_name(pub String);
    #[Scalar]
    impl ScalarType for #scalar_name {
        fn parse(value: Value) -> InputValueResult<Self> {
            match value {
                Value::String(s) => Ok( #scalar_name(s)),
                _ => Err(InputValueError::expected_type(value)),

            }
        }
        fn to_value(&self) -> Value {
            Value::String(self.0.to_string())
        }
    }


    };
    Ok(scalar_def)
}
