use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use toml;

pub struct CustomResolvers {
    pub using: Vec<String>,
    pub bodies: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct CustomResolver {
    pub target_type: String,
    pub body: String,
    pub using: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ResolverSetting {
    pub target_type: String,
    pub target_field: String,
    pub resolver_type: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Additional {
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct RendererConfig {
    pub using: Option<HashMap<String, String>>,
    pub default_data_source_fetch_method: Option<String>,
    pub custom_member_types: Option<Vec<String>>,
    pub resolver: Option<Vec<ResolverSetting>>,
    pub custom_resolver: Option<Vec<CustomResolver>>,
    pub additional: Option<Vec<Additional>>,
}

impl RendererConfig {
    pub fn data_source_using(&self) -> String {
        match self.using.as_ref() {
            Some(using) => using
                .get("data_source")
                .map(|v| v.to_string())
                .unwrap_or_else(|| "use crate::datasource::DataSource".to_string()),
            None => "use crate::datasource::DataSource".to_string(),
        }
    }

    pub fn data_source_fetch_method_from_ctx(&self) -> String {
        match self.default_data_source_fetch_method.as_ref() {
            Some(v) => v.to_string(),
            None => "ctx.data_unchecked::<DataSource>()".to_string(),
        }
    }

    /// if a type contained this set, the field that has the type supposed to be a member instead of resolver method.
    pub fn custom_member_types(&self) -> HashSet<String> {
        match self.custom_member_types.as_ref() {
            None => HashSet::<String>::new(),
            Some(member_types) => member_types.iter().map(|v| v.to_string()).collect(),
        }
    }

    pub fn resolver_setting(&self) -> HashMap<String, HashMap<String, String>> {
        match self.resolver.as_ref() {
            None => return HashMap::new(),
            Some(resolver) => {
                if resolver.is_empty() {
                    return HashMap::new();
                } else {
                    let mut result = HashMap::<String, HashMap<String, String>>::new();
                    for each_resolver in resolver.iter() {
                        let field_and_resolver_type = result
                            .entry(each_resolver.target_type.to_string())
                            .or_insert(HashMap::<String, String>::new());
                        field_and_resolver_type.insert(
                            each_resolver.target_field.to_string(),
                            each_resolver.resolver_type.to_string(),
                        );
                    }
                    result
                }
            }
        }
    }

    pub fn custom_resolvers(&self) -> HashMap<String, CustomResolvers> {
        match self.custom_resolver.as_ref() {
            None => return HashMap::new(),
            Some(custom_resolver) => {
                if custom_resolver.is_empty() {
                    return HashMap::new();
                } else {
                    let mut result = HashMap::<String, CustomResolvers>::new();
                    for custom_resolver in custom_resolver.iter() {
                        let custom_resolvers_field = result
                            .entry(custom_resolver.target_type.to_string())
                            .or_insert(CustomResolvers {
                                using: vec![],
                                bodies: vec![],
                            });
                        custom_resolvers_field
                            .bodies
                            .push(custom_resolver.body.clone());
                        if let Some(using) = custom_resolver.using.as_ref() {
                            custom_resolvers_field.using.push(using.clone());
                        }
                    }
                    result
                }
            }
        }
    }

    pub fn load(file_path: &str) -> Result<RendererConfig> {
        let toml_str: String = fs::read_to_string(file_path)?;
        let config: RendererConfig = toml::from_str(&toml_str).map_err(|e| anyhow!("{}", e))?;
        Ok(config)
    }
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            using: None,
            custom_member_types: None,
            default_data_source_fetch_method: None,
            resolver: None,
            custom_resolver: None,
            additional: None,
        }
    }
}
