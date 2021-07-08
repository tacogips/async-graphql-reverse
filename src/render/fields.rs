use super::super::parse::{self, *};
use super::argument::*;
use super::dependencies::*;
use super::keywords::*;
use super::sorter::sort_by_line_pos;
use super::tokens::*;
use super::typ::*;
use super::RenderContext;
use crate::config::*;
use anyhow::Result;
use heck::SnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::*;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use strum::*;

pub struct FieldsInfo {
    pub members: Vec<TokenStream>,
    pub methods: Vec<TokenStream>,
    pub dependencies: Vec<TokenStream>,
}

impl FieldsInfo {
    fn new() -> Self {
        Self {
            members: vec![],
            methods: vec![],
            dependencies: vec![],
        }
    }
}

struct MemberAndMethod {
    pub member: Option<TokenStream>,
    pub method: Option<TokenStream>,
    pub dependencies: Vec<TokenStream>,
}

pub fn fields_info(
    mut fields: Vec<&parse::Field>,
    schema: &StructuredSchema,
    config: &RendererConfig,
    context: &RenderContext,
    field_resolver: Option<&HashMap<String, &ResolverSetting>>,
    custom_member_types: &HashSet<String>,
) -> Result<FieldsInfo> {
    fields.sort_by(sort_by_line_pos);
    let mut result = FieldsInfo::new();
    for field in fields.iter() {
        let MemberAndMethod {
            member,
            method,
            mut dependencies,
        } = convert_field(
            field,
            schema,
            context,
            config,
            field_resolver,
            custom_member_types,
        )?;

        if let Some(member) = member {
            result.members.push(member)
        }

        if let Some(method) = method {
            result.methods.push(method);
        }

        result.dependencies.append(&mut dependencies);
    }
    Ok(result)
}

fn convert_field(
    field: &parse::Field,
    schema: &StructuredSchema,
    render_context: &RenderContext,
    renderer_config: &RendererConfig,
    field_resolver: Option<&HashMap<String, &ResolverSetting>>,
    custom_member_types: &HashSet<String>,
) -> Result<MemberAndMethod> {
    match field_is_method_or_member(
        &field,
        &schema,
        &render_context,
        &renderer_config,
        &field_resolver,
        &custom_member_types,
    )? {
        ResolverType::Method => {
            resolver_with_datasource(field, schema, render_context, renderer_config)
        }
        ResolverType::Field => resolver_with_member(field, schema, render_context),
    }
}

pub fn field_is_method_or_member(
    field: &parse::Field,
    schema: &StructuredSchema,
    render_context: &RenderContext,
    _renderer_config: &RendererConfig,
    field_resolver: &Option<&HashMap<String, &ResolverSetting>>,
    custom_member_types: &HashSet<String>,
) -> Result<ResolverType> {
    if let Some(field_resolver) = field_resolver {
        if let Some(method_type) = resolver_type_in_resolver_setting(&field.name, &field_resolver) {
            return Ok(method_type);
        }
    }

    if let parse::TypeDef::Object(object) = render_context.parent {
        //TODO(tacogips) more customize if needed
        if schema.is_query(&object.name) {
            return Ok(ResolverType::Method);
        } else if schema.is_mutation(&object.name) {
            return Ok(ResolverType::Method);
        }
    }

    if field_is_a_member(field, schema, custom_member_types)? {
        Ok(ResolverType::Field)
    } else {
        Ok(ResolverType::Method)
    }
}

#[derive(Eq, PartialEq, Debug, EnumString)]
pub enum ResolverType {
    #[strum(serialize = "method")]
    Method,
    #[strum(serialize = "field")]
    Field,
}

fn resolver_type_in_resolver_setting(
    field_name: &str,
    resolver_field_setting: &HashMap<String, &ResolverSetting>,
) -> Option<ResolverType> {
    match resolver_field_setting.get(field_name) {
        Some(ResolverSetting { resolver_type, .. }) => resolver_type
            .as_ref()
            .map(|typ| ResolverType::from_str(typ).unwrap()),
        None => None,
    }
}

/// default:
///  primitive type with no arguments => member
///  others         => method
fn field_is_a_member(
    field: &parse::Field,
    schema: &StructuredSchema,
    custom_member_types: &HashSet<String>,
) -> Result<bool> {
    let source_type = source_type_def(&field.typ, schema)?;
    if let parse::TypeDef::Primitive(_) = source_type {
        if field.arguments.is_empty() {
            Ok(true)
        } else {
            Ok(false)
        }
    } else if let parse::TypeDef::Scalar(_) = source_type {
        if field.arguments.is_empty() {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        if custom_member_types.contains(&source_type.name()) {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// return resolver method
///```
/// pub async field_name(&self, ctx: &Context<'_>,arg1:Arg1, arg2:Arg2) -> ResultType {
///     ctx.data_unchecked::<DataSource>().#resolver_method_name (&self #arg_values)
/// }
///
///```
fn resolver_with_member(
    field: &parse::Field,
    schema: &StructuredSchema,
    context: &RenderContext,
) -> Result<MemberAndMethod> {
    let name = field_or_member_name(field);
    let typ = value_type_def_token(&field.typ, &schema, &context)?;
    let member = Some(quote! { pub #name :#typ });

    let field_rustdoc = match &field.description {
        Some(desc_token) => {
            let comment: TokenStream = format!("///{}", desc_token).parse().unwrap();
            quote! {
                #comment
            }
        }
        None => quote! {},
    };

    let member_need_clone = if let ValueTypeDef::Named(typ) = &field.typ {
        let type_def = typ.as_type_def(&schema.definitions).unwrap();
        match type_def {
            TypeDef::Primitive(PrimitiveKind::Int)
            | TypeDef::Primitive(PrimitiveKind::Float)
            | TypeDef::Primitive(PrimitiveKind::Boolean) => false,
            _ => true,
        }
    } else {
        true
    };

    let resolver_body = if member_need_clone {
        quote! { self.#name.clone() }
    } else {
        quote! { self.#name }
    };

    let method = Some(quote! {
        #field_rustdoc
        pub async fn #name(&self) -> #typ  {
            #resolver_body
        }
    });

    let dependencies = dependency(&field.typ, schema, context)?;

    Ok(MemberAndMethod {
        member,
        method,
        dependencies,
    })
}

pub fn args_defs_and_values(
    field: &parse::Field,
    schema: &StructuredSchema,
    name_prefix: &str,
    context: &RenderContext,
) -> Result<(TokenStream, TokenStream)> {
    if field.arguments.is_empty() {
        Ok((quote! {}, quote! {}))
    } else {
        let arg_defs = field
            .arguments
            .iter()
            .map(|argument| argument_def_token(argument, &schema, &name_prefix, &context))
            .collect::<Result<Vec<TokenStream>>>()?;
        let arg_defs = separate_by_comma(arg_defs);

        let arg_values: Vec<TokenStream> = field
            .arguments
            .iter()
            .map(|arg| {
                let arg = format_ident!("{}", arg.name_string().to_snake_case());
                quote! {#arg}
            })
            .collect();

        let arg_values = separate_by_comma(arg_values);

        Ok((quote! {,#arg_defs}, quote! {,#arg_values}))
    }
}

/// return resolver method
///```
/// pub async field_name(&self, ctx: &Context<'_>,arg1:Arg1, arg2:Arg2) -> ResultType {
///     ctx.data_unchecked::<DataSource>().#resolver_method_name (&self #arg_values).await
/// }
///
///```
fn resolver_with_datasource(
    field: &parse::Field,
    schema: &StructuredSchema,
    context: &RenderContext,
    renderer_config: &RendererConfig,
) -> Result<MemberAndMethod> {
    let (arg_defs, arg_values) = args_defs_and_values(&field, &schema, "", &context)?;

    let field_name = field_or_member_name(field);
    let resolver_method_name = format_ident!(
        "{}",
        format!("{}_{}", context.parent_name(), field.name_string()).to_snake_case()
    );

    let field_rustdoc = match &field.description {
        Some(desc_token) => {
            let comment: TokenStream = format!("///{}", desc_token).parse().unwrap();
            quote! {
                #comment
            }
        }
        None => quote! {},
    };

    let typ = value_type_def_token(&field.typ, &schema, &context)?;
    let typ: TokenStream = quote! {Result<#typ>};
    let data_source_fetch_method: TokenStream = renderer_config
        .data_source_fetch_method_from_ctx()
        .parse()
        .unwrap();
    let method = quote! {
        #field_rustdoc
        pub async fn #field_name(&self, ctx: &Context<'_> #arg_defs ) -> #typ {
            #data_source_fetch_method.#resolver_method_name (&ctx, self #arg_values).await
        }
    };

    let mut dependencies = dependency(&field.typ, schema, context)?;

    for argument in field.arguments.iter() {
        let mut each_deps = dependency(&argument.typ, schema, context)?;
        dependencies.append(&mut each_deps);
    }

    Ok(MemberAndMethod {
        member: None,
        method: Some(method),
        dependencies,
    })
}

fn field_or_member_name(field: &parse::Field) -> Ident {
    let field_name: String = field.name_string().to_snake_case().into();
    if RUST_KEYWORDS.contains(&field_name.as_ref()) {
        format_ident!("r#{}", field_name)
    } else {
        format_ident!("{}", field_name)
    }
}
