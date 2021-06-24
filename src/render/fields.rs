use super::super::parse::{self, *};
use super::argument::*;
use super::config::RendererConfig;
use super::dependencies::*;
use super::keywords::*;
use super::sorter::sort_by_line_pos;
use super::tokens::*;
use super::typ::*;
use super::RenderContext;
use anyhow::Result;
use heck::SnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::*;
use std::collections::HashMap;
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
    field_resolver: Option<&HashMap<String, String>>,
) -> Result<FieldsInfo> {
    fields.sort_by(sort_by_line_pos);
    let mut result = FieldsInfo::new();
    for field in fields.iter() {
        let MemberAndMethod {
            member,
            method,
            mut dependencies,
        } = convert_field(field, schema, config, context, field_resolver)?;

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
    _renderer_config: &RendererConfig,
    render_context: &RenderContext,
    field_resolver: Option<&HashMap<String, String>>,
) -> Result<MemberAndMethod> {
    if let Some(field_resolver) = field_resolver {
        if let Some(resolver_setting) = resolver_setting_of_field(&field.name, &field_resolver) {
            let resolver = match resolver_setting {
                ResolverType::Method => resolver_with_datasource(field, schema, render_context),
                ResolverType::Field => resolver_with_member(field, schema, render_context),
            };
            return resolver;
        }
    }

    if let parse::TypeDef::Object(object) = render_context.parent {
        //TODO(tacogips)  more customize if needed
        if schema.is_query(&object.name) {
            return resolver_with_datasource(field, schema, render_context);
        } else if schema.is_mutation(&object.name) {
            return resolver_with_datasource(field, schema, render_context);
        }
    }

    if field_is_a_member(field, schema)? {
        resolver_with_member(field, schema, render_context)
    } else {
        resolver_with_datasource(field, schema, render_context)
    }
}

#[derive(Eq, PartialEq, Debug, EnumString)]
enum ResolverType {
    #[strum(serialize = "method")]
    Method,
    #[strum(serialize = "field")]
    Field,
}

fn resolver_setting_of_field(
    field_name: &str,
    resolver_field_setting: &HashMap<String, String>,
) -> Option<ResolverType> {
    resolver_field_setting
        .get(field_name)
        .map(|setting| ResolverType::from_str(setting).unwrap())
}

/// default:
///  primitive type with no arguments => member
///  others         => method
fn field_is_a_member(field: &parse::Field, schema: &StructuredSchema) -> Result<bool> {
    match source_type_def(&field.typ, schema)? {
        parse::TypeDef::Primitive(_) => {
            if field.arguments.is_empty() {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
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
    let typ = value_type_def_token(&field.typ, &schema)?;
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

    let method = Some(quote! {
        #field_rustdoc
        pub async fn #name(&self) -> #typ  {
            self.#name.clone()
        }
    });

    let dependencies = dependency(&field.typ, schema, context)?;

    Ok(MemberAndMethod {
        member,
        method,
        dependencies,
    })
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
) -> Result<MemberAndMethod> {
    let (arg_defs, arg_values) = if field.arguments.is_empty() {
        (quote! {}, quote! {})
    } else {
        let arg_defs = field
            .arguments
            .iter()
            .map(|argument| argument_def_token(argument, &schema))
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

        (quote! {,#arg_defs}, quote! {,#arg_values})
    };

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

    let typ = value_type_def_token(&field.typ, &schema)?;
    //TODO() fix Option<Vec<SearchResult>>
    let method = quote! {
        #field_rustdoc
        pub async fn #field_name(&self, ctx: &Context<'_> #arg_defs ) -> #typ {
            ctx.data_unchecked::<DataSource>().#resolver_method_name (self #arg_values).await
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
