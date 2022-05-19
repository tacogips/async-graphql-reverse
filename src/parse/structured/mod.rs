pub mod schema;
use crate::config::*;
pub use schema::*;

use anyhow::Result;
use async_graphql_parser::{types as async_gql_types, Positioned as AsyncGqlPositioned};

macro_rules! node_as_string {
    ($variable:expr) => {
        $variable.node.as_str().to_string()
    };
}

pub fn convert_to_structured_schema(
    service_document: async_gql_types::ServiceDocument,
    config: &RendererConfig,
) -> Result<StructuredSchema> {
    let mut query_name: Option<String> = None;
    let mut mutation_name: Option<String> = None;
    let mut subscription_name: Option<String> = None;

    let mut definitions = Definitions::default();

    for each_node in service_document.definitions {
        match each_node {
            async_gql_types::TypeSystemDefinition::Schema(schema_def) => {
                query_name = schema_def.node.query.map(|query| node_as_string!(query));
                mutation_name = schema_def
                    .node
                    .mutation
                    .map(|mutation| node_as_string!(mutation));

                subscription_name = schema_def
                    .node
                    .subscription
                    .map(|subscription| node_as_string!(subscription));
            }

            async_gql_types::TypeSystemDefinition::Type(type_def) => {
                definitions.add_definition(convert_type_def(type_def, config));
            }

            async_gql_types::TypeSystemDefinition::Directive(directive_def) => {
                log::warn!(
                    "directive not supported yet :{}",
                    directive_def.node.name.node
                );
            }
        }
    }

    Ok(StructuredSchema {
        query_name,
        mutation_name,
        subscription_name,
        definitions,
    })
}

fn convert_type_def(
    type_def: AsyncGqlPositioned<async_gql_types::TypeDefinition>,
    config: &RendererConfig,
) -> Definition {
    let line_pos = type_def.pos.line;
    let type_def = type_def.node;

    let name = node_as_string!(type_def.name);
    let description = type_def.description.map(|desc| node_as_string!(desc));
    let resolver_settings = config.resolver_setting();

    match type_def.kind {
        async_gql_types::TypeKind::Scalar => Definition::Scalar(Scalar { name, line_pos }),
        async_gql_types::TypeKind::Object(object_type) => {
            let fields_resolver_setting = resolver_settings.get(&name);

            let fields = convert_fields(&object_type.fields, fields_resolver_setting);

            let object = Object {
                name,
                fields,
                description,
                line_pos,
                impl_interface_name: object_type
                    .implements
                    .into_iter()
                    .map(|implement| node_as_string!(implement))
                    .collect(),
            };

            Definition::Object(object)
        }
        async_gql_types::TypeKind::Interface(interface) => {
            let fields = convert_fields(&interface.fields, None);

            let intf = Interface {
                name,
                //TODO(tacogips)concrete_type_names  always be empty?
                concrete_type_names: interface
                    .implements
                    .into_iter()
                    .map(|i| node_as_string!(i))
                    .collect(),
                fields,
                line_pos,
                description,
            };

            Definition::Interface(intf)
        }
        async_gql_types::TypeKind::Union(union_type) => {
            let line_pos = union_type
                .members
                .first()
                .map_or(0, |member| member.pos.line);

            let type_names = union_type
                .members
                .into_iter()
                .map(|member| node_as_string!(member))
                .collect();

            let union = Union {
                name,
                type_names,
                line_pos,
                description,
            };

            Definition::Union(union)
        }
        async_gql_types::TypeKind::Enum(enum_type) => {
            let enum_values = enum_type
                .values
                .iter()
                .map(|enum_value| convert_enum_value(enum_value))
                .collect();

            let enum_def = Enum {
                name,
                values: enum_values,
                line_pos,
                description,
            };

            Definition::Enum(enum_def)
        }
        async_gql_types::TypeKind::InputObject(input_type) => {
            let input_fields = input_type
                .fields
                .iter()
                .map(|input_field| convert_input_field_def(input_field))
                .collect();

            let input_object = InputObject {
                name,
                fields: input_fields,
                description,
                line_pos,
            };

            Definition::InputObject(input_object)
        }
    }
}

fn convert_enum_value(
    enum_def: &AsyncGqlPositioned<async_gql_types::EnumValueDefinition>,
) -> EnumValue {
    let enum_def = enum_def.node.clone();

    if !enum_def.directives.is_empty() {
        log::warn!(
            "directive is not supported yet, {}",
            node_as_string!(enum_def.value)
        );
    }

    EnumValue {
        value_name: node_as_string!(enum_def.value),
        description: enum_def.description.map(|desc| node_as_string!(desc)),
    }
}

fn convert_fields(
    fields: &Vec<AsyncGqlPositioned<async_gql_types::FieldDefinition>>,
    fields_resolver_setting: Option<&FieldsResolverSetting>,
) -> Vec<Field> {
    fields
        .iter()
        .map(|field| convert_field_def(field, fields_resolver_setting))
        .collect()
}

fn convert_field_def(
    field_def: &AsyncGqlPositioned<async_gql_types::FieldDefinition>,
    fields_resolver_setting: Option<&FieldsResolverSetting>,
) -> Field {
    let line_pos = field_def.pos.line;
    let field_def = field_def.node.clone();

    if !field_def.directives.is_empty() {
        log::warn!(
            "directive is not supported yet, {}",
            node_as_string!(field_def.name)
        );
    }

    let mut arguments: Vec<Argument> = field_def
        .arguments
        .iter()
        .map(|arg| convert_argument(arg))
        .collect();

    if let Some(fields_resolver_setting) = fields_resolver_setting {
        if let Some(resolver_setting) =
            fields_resolver_setting.get(&node_as_string!(field_def.name))
        {
            if let Some(args) = &resolver_setting.argument {
                let mut additional_args: Vec<Argument> = args
                    .iter()
                    .map(|arg| convert_argument_from_config_arg(arg))
                    .collect();
                arguments.append(&mut additional_args);
            }
        }
    }
    Field {
        name: node_as_string!(field_def.name),
        description: field_def.description.map(|desc| node_as_string!(desc)),
        typ: convert_type_to_value(field_def.ty.node),
        arguments,
        line_pos,
    }
}

pub fn convert_input_field_def(
    input_field_def: &AsyncGqlPositioned<async_gql_types::InputValueDefinition>,
) -> InputField {
    let line_pos = input_field_def.pos.line;
    let input_field_def = input_field_def.node.clone();

    InputField {
        name: node_as_string!(input_field_def.name),
        description: input_field_def
            .description
            .map(|desc| node_as_string!(desc)),
        typ: convert_type_to_value(input_field_def.ty.node),
        line_pos,
    }
}

fn convert_argument(
    input_def: &AsyncGqlPositioned<async_gql_types::InputValueDefinition>,
) -> Argument {
    let input_def = input_def.node.clone();
    if let Some(default_value) = input_def.default_value {
        log::warn!(
            "default value of argument is not supported yet. argument:{} {}",
            node_as_string!(input_def.name),
            default_value
        );
    }

    if !input_def.directives.is_empty() {
        log::warn!(
            "directive is not supported yet. argument:{}",
            node_as_string!(input_def.name)
        );
    }

    Argument {
        name: node_as_string!(input_def.name),
        typ: convert_type_to_value(input_def.ty.node),
        description: input_def.description.map(|desc| node_as_string!(desc)),
    }
}

fn convert_argument_from_config_arg(arg: &ResolverArgument) -> Argument {
    let typ = async_gql_types::Type::new(&arg.arg_type)
        .unwrap_or_else(|| panic!("invalid resolver argument type :{:?}", arg));
    Argument {
        name: arg.arg_name.clone(),
        typ: convert_type_to_value(typ),
        description: arg.arg_description.clone(),
    }
}

pub fn convert_type_to_value(type_def: async_gql_types::Type) -> ValueTypeDef {
    match type_def.base {
        async_gql_types::BaseType::Named(name) => ValueTypeDef::Named(NamedValue {
            value_type_name: name.as_str().to_string(),
            is_nullable: type_def.nullable,
        }),
        async_gql_types::BaseType::List(inner_type) => {
            let inner = convert_type_to_value(*inner_type);

            ValueTypeDef::List(ListValue {
                inner: Box::new(inner),
                is_nullable: type_def.nullable,
            })
        }
    }
}
