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

            parse_schema(&schema_body, config)
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
