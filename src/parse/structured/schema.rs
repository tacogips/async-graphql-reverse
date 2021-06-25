use anyhow::{anyhow, Result};
use derive_macro_tool::{LinePosition, NameString};
use lazy_static::lazy_static;
use paste::paste;
use std::collections::HashMap;
use strum::{AsRefStr, EnumString};

#[derive(Debug)]
pub struct StructuredSchema {
    pub query_name: Option<String>,
    pub mutation_name: Option<String>,
    pub subscription_name: Option<String>,
    pub definitions: Definitions,
}
impl StructuredSchema {
    pub fn is_query(&self, obj_name: &str) -> bool {
        match self.query_name.as_ref() {
            Some(query) => *query == *obj_name,
            None => false,
        }
    }

    pub fn is_mutation(&self, obj_name: &str) -> bool {
        match self.mutation_name.as_ref() {
            Some(mutation) => *mutation == *obj_name,
            None => false,
        }
    }
}

pub enum Definition {
    Scalar(Scalar),
    Object(Object),
    Interface(Interface),
    Union(Union),
    Enum(Enum),
    InputObject(InputObject),
}

#[derive(Debug)]
pub struct Definitions {
    pub input_objects: HashMap<String, InputObject>,
    pub objects: HashMap<String, Object>,
    pub scalars: HashMap<String, Scalar>,
    pub unions: HashMap<String, Union>,
    pub enums: HashMap<String, Enum>,
    pub interfaces: HashMap<String, Interface>,
}

impl Definitions {
    pub fn new() -> Self {
        Self {
            input_objects: HashMap::<String, InputObject>::new(),
            objects: HashMap::<String, Object>::new(),
            scalars: HashMap::<String, Scalar>::new(),
            unions: HashMap::<String, Union>::new(),
            enums: HashMap::<String, Enum>::new(),
            interfaces: HashMap::<String, Interface>::new(),
        }
    }

    pub fn add_definition(&mut self, definition: Definition) {
        match definition {
            Definition::Scalar(v) => {
                self.scalars.insert(v.name_string(), v);
            }
            Definition::Object(v) => {
                self.objects.insert(v.name_string(), v);
            }
            Definition::Interface(v) => {
                self.interfaces.insert(v.name_string(), v);
            }
            Definition::Union(v) => {
                self.unions.insert(v.name_string(), v);
            }
            Definition::Enum(v) => {
                self.enums.insert(v.name_string(), v);
            }
            Definition::InputObject(v) => {
                self.input_objects.insert(v.name_string(), v);
            }
        }
    }
}

pub trait NameString {
    fn name_string(&self) -> String;
}

pub trait LinePosition {
    fn line_position(&self) -> usize;
}

#[derive(Debug, NameString, LinePosition)]
pub struct Scalar {
    pub name: String,
    pub line_pos: usize,
}

#[derive(Debug, NameString, LinePosition)]
pub struct Enum {
    pub name: String,
    pub values: Vec<EnumValue>,
    pub line_pos: usize,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct EnumValue {
    pub value_name: String,
    pub description: Option<String>,
}

#[derive(Debug, NameString, LinePosition)]
pub struct Union {
    pub name: String,
    //TODO() rename to concrete_type_names
    pub type_names: Vec<String>,
    pub line_pos: usize,
    pub description: Option<String>,
}

#[derive(Debug, NameString, LinePosition)]
pub struct Interface {
    pub name: String,
    //TODO(tacogips)concrete_type_names  always be empty?
    pub concrete_type_names: Vec<String>,
    pub fields: Vec<Field>,
    pub description: Option<String>,
    pub line_pos: usize,
}

#[derive(Debug, NameString, LinePosition)]
pub struct InputObject {
    pub name: String,
    pub fields: Vec<InputField>,
    pub description: Option<String>,
    pub line_pos: usize,
}

#[derive(Debug, NameString, LinePosition)]
pub struct Object {
    pub name: String,
    pub fields: Vec<Field>,
    pub description: Option<String>,
    pub line_pos: usize,
    pub impl_interface_name: Vec<String>,
}

#[derive(Debug, NameString, LinePosition)]
pub struct Field {
    pub name: String,
    pub description: Option<String>,
    pub typ: ValueTypeDef,
    pub arguments: Vec<Argument>,
    pub line_pos: usize,
}

#[derive(Debug, NameString)]
pub struct Argument {
    pub name: String,
    pub typ: ValueTypeDef,
    pub description: Option<String>,
    //TODO(tacogips) default value not supported
    //pub default_value: Option<String>,
}

#[derive(Debug, NameString, LinePosition)]
pub struct InputField {
    pub name: String,
    pub description: Option<String>,
    pub typ: ValueTypeDef,
    pub line_pos: usize,
}

#[derive(Debug)]
pub enum ValueTypeDef {
    Named(NamedValue),
    List(ListValue),
}

impl ValueTypeDef {
    pub fn nullable(&self) -> bool {
        match self {
            ValueTypeDef::Named(v) => v.is_nullable,
            ValueTypeDef::List(v) => v.is_nullable,
        }
    }
}

#[derive(Debug)]
pub struct NamedValue {
    pub value_type_name: String,
    pub is_nullable: bool,
}

impl NamedValue {
    pub fn as_type_def<'a>(&self, definitions: &'a Definitions) -> Result<TypeDef<'a>> {
        let type_name = &self.value_type_name;

        let result = if let Some(primitive) = PRIMITIVE_KIND_MAP.get(type_name.as_str()) {
            TypeDef::Primitive(primitive)
        } else if let Some(input_object) = definitions.input_objects.get(type_name) {
            TypeDef::InputObject(input_object)
        } else if let Some(object) = definitions.objects.get(type_name) {
            TypeDef::Object(object)
        } else if let Some(scalar) = definitions.scalars.get(type_name) {
            TypeDef::Scalar(scalar)
        } else if let Some(union) = definitions.unions.get(type_name) {
            TypeDef::Union(union)
        } else if let Some(enm) = definitions.enums.get(type_name) {
            TypeDef::Enum(enm)
        } else if let Some(interface) = definitions.interfaces.get(type_name) {
            TypeDef::Interface(interface)
        } else {
            return Err(anyhow!("type: {} not defined", type_name));
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub struct ListValue {
    pub inner: Box<ValueTypeDef>,
    pub is_nullable: bool,
}

macro_rules! is {
    ($v:ident) => {
        paste! {
            pub fn [<is_ $v:snake>] (&self) -> bool {
                if let TypeDef::$v(_) = self {
                    true
                } else {
                    false
                }
            }
        }
    };
}

#[derive(Debug)]
pub enum TypeDef<'a> {
    Primitive(&'a PrimitiveKind),
    Object(&'a Object),
    Enum(&'a Enum),
    InputObject(&'a InputObject),
    Scalar(&'a Scalar),
    Union(&'a Union),
    Interface(&'a Interface),
}
impl<'a> TypeDef<'a> {
    is! {Primitive}
    is! {Object}
    is! {Enum}
    is! {InputObject}
    is! {Scalar}
    is! {Union}
    is! {Interface}

    pub fn name(&self) -> String {
        match self {
            TypeDef::Primitive(v) => v.rust_type(),
            TypeDef::Object(v) => v.name.to_string(),
            TypeDef::Enum(v) => v.name.to_string(),
            TypeDef::InputObject(v) => v.name.to_string(),
            TypeDef::Scalar(v) => v.name.to_string(),
            TypeDef::Union(v) => v.name.to_string(),
            TypeDef::Interface(v) => v.name.to_string(),
        }
    }
}

#[derive(AsRefStr, EnumString, Debug)]
pub enum PrimitiveKind {
    #[strum(serialize = "Int")]
    Int,
    #[strum(serialize = "Float")]
    Float,
    #[strum(serialize = "String")]
    Str,
    #[strum(serialize = "Boolean")]
    Boolean,
    #[strum(serialize = "ID")]
    ID,
}

impl PrimitiveKind {
    pub fn rust_type(&self) -> String {
        match self {
            PrimitiveKind::Int => "i64".to_string(),
            PrimitiveKind::Float => "f64".to_string(),
            PrimitiveKind::Str => "String".to_string(),
            PrimitiveKind::Boolean => "bool".to_string(),
            PrimitiveKind::ID => "ID".to_string(),
        }
    }
}

lazy_static! {
    static ref PRIMITIVE_KIND_MAP: HashMap<&'static str, PrimitiveKind> = {
        let mut m = HashMap::new();
        m.insert(PrimitiveKind::Int.as_ref(), PrimitiveKind::Int);
        m.insert(PrimitiveKind::Float.as_ref(), PrimitiveKind::Float);
        m.insert(PrimitiveKind::Str.as_ref(), PrimitiveKind::Str);
        m.insert(PrimitiveKind::Boolean.as_ref(), PrimitiveKind::Boolean);
        m.insert(PrimitiveKind::ID.as_ref(), PrimitiveKind::ID);
        m
    };
}

pub fn source_type_def<'a>(
    type_def: &ValueTypeDef,
    schema: &'a StructuredSchema,
) -> Result<TypeDef<'a>> {
    match type_def {
        ValueTypeDef::Named(named_value) => named_value.as_type_def(&schema.definitions),
        ValueTypeDef::List(list_value) => source_type_def(&list_value.inner, &schema),
    }
}
