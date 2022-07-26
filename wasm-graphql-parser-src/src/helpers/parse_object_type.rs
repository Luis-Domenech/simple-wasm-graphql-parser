use apollo_parser::ast::{ObjectTypeDefinition, ObjectTypeExtension};
use crate::prelude::{ObjectType, REGEX, parse_directive_instance, parse_field, FieldDefs, Config, DirectiveData};

pub fn parse_object_type_def(object_type_def: ObjectTypeDefinition, config: &Config) -> ObjectType {

    let mut name = object_type_def.name().unwrap().text().to_string();
    name = REGEX.match_whitespace.replace_all(&name, "").to_string();

    let object_type_type = object_type_def.type_token().unwrap().text().to_string();

    let description = match object_type_def.description() {
        Some(desc) => Some(REGEX.match_description_beginning_end_quotes.replace_all(&desc.to_string(), "").to_string()),
        None => None,
    };

    let directives: Option<Vec<DirectiveData>> = parse_directive_instance(object_type_def.directives(), config);
    let mut implements: Vec<String> = vec![];
    
    let mut object_type = ObjectType {
        name: name,
        object_type: object_type_type,
        fields: vec![],
        description: description,
        directives: directives,
        implements: None
    };

    for field_def in object_type_def.fields_definition().unwrap().field_definitions() {
        object_type.fields.push(parse_field(FieldDefs::FieldDef(field_def), config));
    }

    for implement in object_type_def.implements_interfaces().iter() {
        // We will just store name of the interface being implemented. We don't need to store data we already have.
        for named_type in implement.named_types() {
            let implement_name = REGEX.match_word.captures(&named_type.name().unwrap().to_string()).unwrap().unwrap()[0].to_string();
            implements.push(implement_name);
        }
    }

    if implements.len() > 0 {
        object_type.implements = Some(implements);
    }


    return object_type;
}


// Here we iterate the object types we created and just directly append the info from the extension
pub fn parse_object_type_extension(object_type_extension: ObjectTypeExtension, object_types: &mut Vec<ObjectType>, config: &Config) {
    let mut extends = object_type_extension.name().unwrap().to_string();
    extends = REGEX.match_whitespace.replace_all(&extends, "").to_string();

    // If no match is found, then we simply do nothing this this extension. We won't return an error
    for object_type in object_types {
        if extends == object_type.name {
            match parse_directive_instance(object_type_extension.directives(), config) {
                Some(directives) => {
                    for directive in directives {
                        if let Some(ref mut d) = object_type.directives {
                            d.push(directive);
                        }
                        else {
                            object_type.directives = Some(vec![directive]);
                        }
                    }
                },
                None => {}
            }

            for field_def in object_type_extension.fields_definition().unwrap().field_definitions() {
                object_type.fields.push(parse_field(FieldDefs::FieldDef(field_def), config));
            }

            let implements: Option<String> = if object_type_extension.implements_interfaces().is_some() { Some(object_type_extension.implements_interfaces().unwrap().to_string()) } else { None };

            if implements.is_some() {
                if let Some(ref mut i) = object_type.implements {
                    i.push(implements.unwrap());
                }
                else {
                    object_type.implements = Some(vec![implements.unwrap()]);
                }
            }
        }
    }
}