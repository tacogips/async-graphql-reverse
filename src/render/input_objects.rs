use super::super::parse::*;
use super::comment::*;
use super::dependencies::*;
use super::files::{fmt_file, pathbuf_to_str};
use super::input_fields::*;
use super::sorter::sort_by_line_pos_and_name;
use super::tokens::*;
use super::RenderContext;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::*;
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn write_input_objects(output_dir: &str, structured_schema: &StructuredSchema) -> Result<bool> {
    let mut output_file = PathBuf::from(output_dir);
    output_file.push("input_objects.rs");
    let file_path_str = pathbuf_to_str(&output_file);

    if output_file.exists() {
        fs::remove_file(&file_path_str)?;
    }

    let mut input_objects: Vec<&InputObject> = structured_schema
        .definitions
        .input_objects
        .values()
        .into_iter()
        .collect();
    if input_objects.is_empty() {
        return Ok(false);
    }
    input_objects.sort_by(sort_by_line_pos_and_name);

    let mut all_dependencies = HashSet::<String>::new();
    let mut object_defs = Vec::<String>::new();

    for each_obj in input_objects {
        let (object_token, dependencies) = input_object_token(each_obj, &structured_schema)?;

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

    dest_file.write(FILE_HEADER_COMMENT.as_bytes())?;
    let header = quote! {
        use async_graphql::*;
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

fn input_object_token(
    input_object: &InputObject,
    schema: &StructuredSchema,
) -> Result<(TokenStream, Vec<TokenStream>)> {
    let object_name = format_ident!("{}", input_object.name);
    let comment = match &input_object.description {
        Some(desc_token) => to_rust_docs_token(desc_token),
        None => quote! {},
    };

    let context = RenderContext {
        parent: TypeDef::InputObject(input_object),
    };

    let InputFieldsInfo {
        members,
        dependencies,
    } = input_fields_info(input_object.fields.iter().collect(), schema, &context)?;

    let members = separate_by_comma(members);
    let object_def = quote! {
        #comment
        #[derive(InputObject)]
        pub struct #object_name{
            #members
        }

    };
    Ok((object_def, dependencies))
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::RendererConfig;

    #[test]
    pub fn parse_input_1() {
        let schema = r#"
        input SampleInput {
          id: String
          rec:[Int],
        }
        "#;

        let structured_schema = parse_schema(schema, &RendererConfig::default()).unwrap();

        let mut input_objects: Vec<&InputObject> = structured_schema
            .definitions
            .input_objects
            .values()
            .into_iter()
            .collect();
        assert_eq!(1, input_objects.len());
        let input_object = input_objects.remove(0);
        let (object_token, _dependencies) =
            input_object_token(input_object, &structured_schema).unwrap();

        let expected = r#"
    #[derive(InputObject)]
    pub struct SampleInput{
        pub id:Option<String>,
        pub rec:Option<Vec<Option<i64>>>}
"#;
        assert_eq!(
            object_token.to_string().replace(" ", ""),
            expected.to_string().replace("\n", "").replace(" ", "")
        );
    }

    #[test]
    pub fn parse_input_2() {
        let schema = r#"
        input SampleInput {
          id: String
          rec:[Int]!,
        }
        "#;

        let structured_schema = parse_schema(schema, &RendererConfig::default()).unwrap();

        let mut input_objects: Vec<&InputObject> = structured_schema
            .definitions
            .input_objects
            .values()
            .into_iter()
            .collect();
        assert_eq!(1, input_objects.len());
        let input_object = input_objects.remove(0);
        let (object_token, _dependencies) =
            input_object_token(input_object, &structured_schema).unwrap();

        let expected = r#"
    #[derive(InputObject)]
    pub struct SampleInput{
        pub id:Option<String>,
        pub rec:Vec<Option<i64>>}
"#;
        assert_eq!(
            object_token.to_string().replace(" ", ""),
            expected.to_string().replace("\n", "").replace(" ", "")
        );
    }

    #[test]
    pub fn parse_input_3() {
        let schema = r#"
        input SampleInput {
          id: String
          rec:[Int!]!,
        }
        "#;

        let structured_schema = parse_schema(schema, &RendererConfig::default()).unwrap();

        let mut input_objects: Vec<&InputObject> = structured_schema
            .definitions
            .input_objects
            .values()
            .into_iter()
            .collect();
        assert_eq!(1, input_objects.len());
        let input_object = input_objects.remove(0);
        let (object_token, _dependencies) =
            input_object_token(input_object, &structured_schema).unwrap();

        let expected = r#"
    #[derive(InputObject)]
    pub struct SampleInput{
        pub id:Option<String>,
        pub rec:Vec<i64>}
"#;
        assert_eq!(
            object_token.to_string().replace(" ", ""),
            expected.to_string().replace("\n", "").replace(" ", "")
        );
    }
}
