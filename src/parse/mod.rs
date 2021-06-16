use anyhow::{anyhow, Result};

use async_graphql_parser::types::ServiceDocument;
use std::fs;

pub fn parse_schema_file(path: &str) -> Result<ServiceDocument> {
    match fs::read_to_string(path) {
        Ok(schema_body) => parse_schema(&schema_body),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

pub fn parse_schema(schema_body: &str) -> Result<ServiceDocument> {
    async_graphql_parser::parse_schema(schema_body).map_err(|e| anyhow!("{}", e))
}
