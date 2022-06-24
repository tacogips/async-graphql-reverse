pub use super::structured::*;
use crate::config::{Ignore, RendererConfig};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub fn remove_ignored_from_structure(
    structured_schema: &mut StructuredSchema,
    config: &RendererConfig,
) -> Result<()> {
    match &config.ignore {
        None => Ok(()),
        Some(ignore) => {
            if ignore.is_empty() {
                Ok(())
            } else {
                inner_remove_ignored_from_structure(structured_schema, &ignore)
            }
        }
    }
}

macro_rules! is_ignore_type {
    ($typ:ident,
      $object_names_set:ident,
      $enum_names_set:ident,
      $input_object_names_set:ident,
      $scalar_names_set:ident,
      $union_names_set:ident,
      $inerface_names_set:ident) => {
        match $typ {
            TypeDef::Object(Object { name, .. }) => $object_names_set.contains(&name),
            TypeDef::Enum(Enum { name, .. }) => $enum_names_set.contains(&name),
            TypeDef::InputObject(InputObject { name, .. }) => {
                $input_object_names_set.contains(&name)
            }
            TypeDef::Scalar(Scalar { name, .. }) => $scalar_names_set.contains(&name),
            TypeDef::Union(Union { name, .. }) => $union_names_set.contains(&name),
            TypeDef::Interface(Interface { name, .. }) => $inerface_names_set.contains(&name),
            _ => false,
        }
    };
}

macro_rules! accumulate_remove_target {
    ( $component:ident,
      $each_field:ident,
      $target_map:ident,
      $structured_schema:ident,
      $object_names_set:ident,
      $enum_names_set:ident,
      $input_object_names_set:ident,
      $scalar_names_set:ident,
      $union_names_set:ident,
      $inerface_names_set:ident
     ) => {{
        match $each_field
            .typ
            .element_value_type_def(&$structured_schema.definitions)
        {
            Err(e) => {
                println!(
                    "WARN: ignore element not found. {:?}, error {}",
                    $each_field.typ, e
                );
            }
            Ok(typ) => {
                let is_ignore_target = is_ignore_type!(
                    typ,
                    $object_names_set,
                    $enum_names_set,
                    $input_object_names_set,
                    $scalar_names_set,
                    $union_names_set,
                    $inerface_names_set
                );

                if is_ignore_target {
                    $target_map
                        .entry($component.name.clone())
                        .or_insert(HashSet::new())
                        .insert($each_field.name.clone());
                }
            }
        }
    }};
}

fn inner_remove_ignored_from_structure(
    structured_schema: &mut StructuredSchema,
    ignore: &Ignore,
) -> Result<()> {
    let mut ignore_enums_set: HashSet<&String> = HashSet::new();
    let mut ignore_object_set: HashSet<&String> = HashSet::new();
    let mut ignore_input_object_set: HashSet<&String> = HashSet::new();
    let mut ignore_union_set: HashSet<&String> = HashSet::new();
    let mut ignore_interface_set: HashSet<&String> = HashSet::new();
    let mut ignore_scalar_set: HashSet<&String> = HashSet::new();

    if let Some(ignore_enums) = &ignore.r#enum {
        ignore_enums_set = ignore_enums.iter().collect();
    }

    if let Some(ignore_objects) = &ignore.object {
        ignore_object_set = ignore_objects.iter().collect();
    }

    if let Some(ignore_input_object) = &ignore.input_object {
        ignore_input_object_set = ignore_input_object.iter().collect();
    }

    if let Some(ignore_union) = &ignore.union {
        ignore_union_set = ignore_union.iter().collect();
    }

    if let Some(ignore_intrerface) = &ignore.interface {
        ignore_interface_set = ignore_intrerface.iter().collect();
    }

    if let Some(ignore_scalar) = &ignore.scalar {
        ignore_scalar_set = ignore_scalar.iter().collect();
    }

    // === accumlate remove object fields =====================================================================
    let mut remove_object_field_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut remove_object_field_argument_map: HashMap<String, HashMap<String, HashSet<String>>> =
        HashMap::new();
    for object in structured_schema.definitions.objects.values() {
        for each_field in object.fields.iter() {
            accumulate_remove_target!(
                object,
                each_field,
                remove_object_field_map,
                structured_schema,
                ignore_object_set,
                ignore_enums_set,
                ignore_input_object_set,
                ignore_scalar_set,
                ignore_union_set,
                ignore_interface_set
            );

            for argument in each_field.arguments.iter() {
                match argument
                    .typ
                    .element_value_type_def(&structured_schema.definitions)
                {
                    Err(e) => {
                        println!(
                            "WARN: could not ignore argument. element type {:?}, error {}",
                            argument.typ, e
                        );
                    }
                    Ok(typ) => {
                        let is_ignore_target = is_ignore_type!(
                            typ,
                            ignore_object_set,
                            ignore_enums_set,
                            ignore_input_object_set,
                            ignore_scalar_set,
                            ignore_union_set,
                            ignore_interface_set
                        );

                        if is_ignore_target {
                            remove_object_field_argument_map
                                .entry(object.name.clone())
                                .or_insert(HashMap::new())
                                .entry(each_field.name.clone())
                                .or_insert(HashSet::new())
                                .insert(argument.name.clone());
                        }
                    }
                }
            }
        }
    }

    // === accumulate remove input object fields =====================================================================
    let mut remove_input_object_field_map: HashMap<String, HashSet<String>> = HashMap::new();
    for input_object in structured_schema.definitions.input_objects.values() {
        for each_field in input_object.fields.iter() {
            accumulate_remove_target!(
                input_object,
                each_field,
                remove_input_object_field_map,
                structured_schema,
                ignore_object_set,
                ignore_enums_set,
                ignore_input_object_set,
                ignore_scalar_set,
                ignore_union_set,
                ignore_interface_set
            );
        }
    }

    // remove object and field

    for object in structured_schema.definitions.objects.values_mut() {
        remove_object_field_argument_map
            .get(&object.name)
            .map(|remove_field_argument_map| {
                for each_field in object.fields.iter_mut() {
                    let field_name = each_field.name.clone();
                    if let Some(remove_arguments) = remove_field_argument_map.get(&field_name) {
                        each_field
                            .arguments
                            .retain(|argument| !remove_arguments.contains(&argument.name));
                    }
                }
            });

        remove_object_field_map
            .get(&object.name)
            .map(|remove_fields| {
                object
                    .fields
                    .retain(|field| !remove_fields.contains(&field.name));
            });
    }

    // remove input object and field
    for input_object in structured_schema.definitions.input_objects.values_mut() {
        remove_input_object_field_map
            .get(&input_object.name)
            .map(|remove_fields| {
                input_object
                    .fields
                    .retain(|field| !remove_fields.contains(&field.name));
            });
    }

    // remove from definitions
    structured_schema
        .definitions
        .enums
        .retain(|name, _| !ignore_enums_set.contains(name));

    structured_schema
        .definitions
        .objects
        .retain(|name, _| !ignore_object_set.contains(name));

    structured_schema
        .definitions
        .input_objects
        .retain(|name, _| !ignore_input_object_set.contains(name));

    structured_schema
        .definitions
        .unions
        .retain(|name, _| !ignore_union_set.contains(name));

    structured_schema
        .definitions
        .interfaces
        .retain(|name, _| !ignore_interface_set.contains(name));

    structured_schema
        .definitions
        .scalars
        .retain(|name, _| !ignore_scalar_set.contains(name));

    Ok(())
}
