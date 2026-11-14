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
pub struct AdditionalResolver {
    pub target_type: String,
    pub body: String,
    pub using: Option<String>,
}

pub struct HiddenFields {
    pub using: Vec<String>,
    pub field_defs: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EnumSetting {
    pub target_enum: String,
    pub rename_items: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct HiddenField {
    pub target_type: String,
    pub field_def: String,
    pub using: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResolverArgument {
    pub arg_name: String,
    pub arg_type: String,
    pub arg_description: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResolverSetting {
    pub target_type: String,
    pub target_field: String,
    pub resolver_type: Option<String>,
    pub attribute: Option<String>,
    pub argument: Option<Vec<ResolverArgument>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FieldSetting {
    pub target_type: String,
    pub target_field: String,
    pub replace_field_type: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Additional {
    pub body: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Ignore {
    pub r#enum: Option<HashSet<String>>,
    pub object: Option<HashSet<String>>,
    pub input_object: Option<HashSet<String>>,
    pub union: Option<HashSet<String>>,
    pub interface: Option<HashSet<String>>,
    pub scalar: Option<HashSet<String>>,
}

macro_rules! return_false_if_not_empty_set {
    ($set:expr) => {
        if let Some(set) = $set {
            if !set.is_empty() {
                return false;
            }
        }
    };
}

impl Ignore {
    pub fn is_empty(&self) -> bool {
        return_false_if_not_empty_set!(&self.r#enum);
        return_false_if_not_empty_set!(&self.object);
        return_false_if_not_empty_set!(&self.input_object);
        return_false_if_not_empty_set!(&self.union);
        return_false_if_not_empty_set!(&self.interface);
        return_false_if_not_empty_set!(&self.scalar);

        true
    }
}

pub type DefinedEnumName = String;
pub type DefinedTypeName = String;
pub type DefinedFieldName = String;

pub type FieldsResolverSetting<'a> = HashMap<DefinedFieldName, &'a ResolverSetting>;
pub type FieldsSetting<'a> = HashMap<DefinedFieldName, &'a FieldSetting>;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    Objects,
    InputObjects,
    Unions,
    Scalars,
    Interfaces,
    Enums,
}

#[derive(Deserialize, Default, Debug)]
pub struct RendererConfig {
    pub using: Option<HashMap<String, String>>,
    #[serde(default = "RendererConfig::default_data_source_fetch_method_from_ctx")]
    pub data_source_fetch_method: String,
    pub custom_member_types: Option<Vec<String>>,
    pub resolver: Option<Vec<ResolverSetting>>,
    pub additional_resolver: Option<Vec<AdditionalResolver>>,
    pub hidden_field: Option<Vec<HiddenField>>,
    pub additional: Option<Vec<Additional>>,
    pub ignore: Option<Ignore>,
    pub r#enum: Option<Vec<EnumSetting>>,
    pub field: Option<Vec<FieldSetting>>,
    pub enum_rename_items: Option<String>,

    /// With this you can override the header included at the top of the file.
    #[serde(default = "RendererConfig::default_header")]
    pub header: String,

    /// Rather than determining the resolver type based on the type of the field in the
    /// object, we will use this type instead. Overrides specified in
    /// additional_resolver override this setting.
    pub resolver_type: Option<String>,

    /// Additional attributes to apply to the generated object types.
    pub additional_attributes: Option<String>,

    /// By default all generation phases are executed. If set, only the specified phases
    /// will be executed.
    #[serde(default)]
    pub phases: Vec<Phase>,

    /// If set, the Object implementation will not be generated for objects.
    #[serde(default)]
    pub no_object_impl: bool,

    /// If set, dependencies of objects will not be imported. This might be useful if
    /// you have the Scalars phase disabled because you are using your own scalar types.
    #[serde(default)]
    pub no_dependency_imports: bool,
}

impl RendererConfig {
    fn default_header() -> String {
        "use async_graphql::*; use crate::datasource::DataSource;".to_string()
    }

    fn default_data_source_fetch_method_from_ctx() -> String {
        "ctx.data_unchecked::<DataSource>()".to_string()
    }

    /// if a type contained this set, the field that has the type supposed to be a member instead of resolver method.
    pub fn custom_member_types(&self) -> HashSet<DefinedTypeName> {
        match self.custom_member_types.as_ref() {
            None => HashSet::<DefinedTypeName>::new(),
            Some(member_types) => member_types.iter().map(|v| v.to_string()).collect(),
        }
    }

    pub fn resolver_setting(&self) -> HashMap<DefinedTypeName, FieldsResolverSetting> {
        match self.resolver.as_ref() {
            None => return HashMap::new(),
            Some(resolver) => {
                if resolver.is_empty() {
                    return HashMap::new();
                } else {
                    let mut result = HashMap::<DefinedTypeName, FieldsResolverSetting>::new();
                    for each_resolver in resolver.iter() {
                        let field_and_resolver_type = result
                            .entry(each_resolver.target_type.to_string())
                            .or_insert(FieldsResolverSetting::new());
                        field_and_resolver_type
                            .insert(each_resolver.target_field.to_string(), each_resolver);
                    }
                    result
                }
            }
        }
    }

    pub fn field_setting(&self) -> HashMap<DefinedTypeName, FieldsSetting> {
        match self.field.as_ref() {
            None => return HashMap::new(),
            Some(resolver) => {
                if resolver.is_empty() {
                    return HashMap::new();
                } else {
                    let mut result = HashMap::<DefinedTypeName, FieldsSetting>::new();
                    for each_field in resolver.iter() {
                        let field_and_resolver_type = result
                            .entry(each_field.target_type.to_string())
                            .or_insert(FieldsSetting::new());

                        field_and_resolver_type
                            .insert(each_field.target_field.to_string(), each_field);
                    }
                    result
                }
            }
        }
    }

    pub fn enum_settings(&self) -> HashMap<DefinedEnumName, EnumSetting> {
        match self.r#enum.as_ref() {
            None => HashMap::new(),
            Some(enum_settings) => {
                if enum_settings.is_empty() {
                    HashMap::new()
                } else {
                    enum_settings
                        .into_iter()
                        .map(|each_enum| (each_enum.target_enum.to_string(), each_enum.clone()))
                        .collect::<HashMap<DefinedEnumName, EnumSetting>>()
                }
            }
        }
    }

    pub fn hidden_fields(&self) -> HashMap<DefinedTypeName, HiddenFields> {
        match self.hidden_field.as_ref() {
            None => return HashMap::new(),
            Some(hidden_field) => {
                if hidden_field.is_empty() {
                    return HashMap::new();
                } else {
                    let mut result = HashMap::<DefinedTypeName, HiddenFields>::new();
                    for each_hidden_field in hidden_field.iter() {
                        let hidden_field = result
                            .entry(each_hidden_field.target_type.to_string())
                            .or_insert(HiddenFields {
                                using: vec![],
                                field_defs: vec![],
                            });
                        hidden_field
                            .field_defs
                            .push(each_hidden_field.field_def.clone());
                        if let Some(using) = each_hidden_field.using.as_ref() {
                            hidden_field.using.push(using.clone());
                        }
                    }
                    result
                }
            }
        }
    }

    pub fn additional_resolvers(&self) -> HashMap<DefinedTypeName, CustomResolvers> {
        match self.additional_resolver.as_ref() {
            None => return HashMap::new(),
            Some(additional_resolver) => {
                if additional_resolver.is_empty() {
                    return HashMap::new();
                } else {
                    let mut result = HashMap::<DefinedTypeName, CustomResolvers>::new();
                    for custom_resolver in additional_resolver.iter() {
                        let custom_resolvers = result
                            .entry(custom_resolver.target_type.to_string())
                            .or_insert(CustomResolvers {
                                using: vec![],
                                bodies: vec![],
                            });
                        custom_resolvers.bodies.push(custom_resolver.body.clone());
                        if let Some(using) = custom_resolver.using.as_ref() {
                            custom_resolvers.using.push(using.clone());
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
