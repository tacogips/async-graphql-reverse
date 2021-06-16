pub mod structured;
pub use structured::*;

use anyhow::{anyhow, Result};

use std::fs;

pub fn parse_schema_file(path: &str) -> Result<StructuredSchema> {
    match fs::read_to_string(path) {
        Ok(schema_body) => parse_schema(&schema_body),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

pub fn parse_schema(schema_body: &str) -> Result<StructuredSchema> {
    match async_graphql_parser::parse_schema(schema_body) {
        Ok(schema) => convert_to_structured_schema(schema),
        Err(e) => Err(anyhow!("{}", e)),
    }
}
