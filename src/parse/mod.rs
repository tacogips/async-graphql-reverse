pub mod ignoring;
pub mod structured;
use crate::config::RendererConfig;
pub use structured::*;

use anyhow::{anyhow, Result};

use std::fs;

pub fn parse_schema_file(path: &str, config: &RendererConfig) -> Result<StructuredSchema> {
    match fs::read_to_string(path) {
        Ok(mut schema_body) => {
            if let Some(additionals) = &config.additional {
                let merged_additional = additionals
                    .iter()
                    .map(|each| each.body.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");

                schema_body = format!("{} {}", schema_body, merged_additional);
            }

            let mut schema = parse_schema(&schema_body, config)?;

            ignoring::remove_ignored_from_structure(&mut schema, &config)?;
            Ok(schema)
        }
        Err(e) => Err(anyhow!("{}", e)),
    }
}
pub fn parse_schema(schema_body: &str, config: &RendererConfig) -> Result<StructuredSchema> {
    match async_graphql_parser::parse_schema(schema_body) {
        Ok(schema) => convert_to_structured_schema(schema, config),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    pub fn parse_schema_input_1() {
        let schema = r#"
        input SampleInput {
          id: String
          rec:[Int],
        }
        "#;
        let result = parse_schema(schema, &RendererConfig::default()).unwrap();
        let mut definitions = Definitions::default();
        definitions.input_objects.insert(
            "SampleInput".to_string(),
            InputObject {
                name: "SampleInput".to_string(),
                fields: vec![
                    InputField {
                        name: "id".to_string(),
                        description: None,
                        typ: ValueTypeDef::Named(NamedValue {
                            value_type_name: "String".to_string(),
                            is_nullable: true,
                        }),
                        line_pos: 3,
                    },
                    InputField {
                        name: "rec".to_string(),
                        description: None,
                        typ: ValueTypeDef::List(ListValue {
                            inner: Box::new(ValueTypeDef::Named(NamedValue {
                                value_type_name: "Int".to_string(),
                                is_nullable: true,
                            })),
                            is_nullable: true,
                        }),
                        line_pos: 4,
                    },
                ],
                description: None,
                line_pos: 2,
            },
        );

        let expected = StructuredSchema {
            query_name: None,
            mutation_name: None,
            subscription_name: None,
            definitions,
        };

        assert_eq!(result, expected);
    }
}
