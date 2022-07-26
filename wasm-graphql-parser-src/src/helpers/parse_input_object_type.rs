use apollo_parser::ast::{InputObjectTypeDefinition, InputObjectTypeExtension};
use crate::prelude::{ObjectType, DirectiveData, REGEX, parse_directive_instance, parse_field, FieldDefs, Config};

pub fn parse_input_object_type_def(input_object_type_def: InputObjectTypeDefinition, config: &Config) -> ObjectType {

    let mut name = input_object_type_def.name().unwrap().text().to_string();
    name = REGEX.match_whitespace.replace_all(&name, "").to_string();
    
    let input_object_type_type = "input".to_string();

    let description = match input_object_type_def.description() {
        Some(desc) => Some(REGEX.match_description_beginning_end_quotes.replace_all(&desc.to_string(), "").to_string()),
        None => None,
    };

    let directives: Option<Vec<DirectiveData>> = parse_directive_instance(input_object_type_def.directives(), config);
    
    let mut input_object_type = ObjectType {
        name: name,
        object_type: input_object_type_type,
        fields: vec![],
        description: description,
        directives: directives,
        implements: None,
    };

    for field_def in input_object_type_def.input_fields_definition().unwrap().input_value_definitions() {
        input_object_type.fields.push(parse_field(FieldDefs::InputValueDef(field_def), config));
    }

    return input_object_type;
}

// Here we iterate the object types we created and just directly append the info from the extension
pub fn parse_input_object_type_extension(input_object_type_extension: InputObjectTypeExtension, input_object_types: &mut Vec<ObjectType>, config: &Config) {
    let mut extends = input_object_type_extension.name().unwrap().to_string();
    extends = REGEX.match_whitespace.replace_all(&extends, "").to_string();

    // If no match is found, then we simply do nothing this this extension. We won't return an error
    for input_object_type in input_object_types {
        if extends == input_object_type.name {
            match parse_directive_instance(input_object_type_extension.directives(), config) {
                Some(directives) => {
                    for directive in directives {
                        if let Some(ref mut d) = input_object_type.directives {
                            d.push(directive);
                        }
                        else {
                            input_object_type.directives = Some(vec![directive]);
                        }
                    }
                },
                None => {}
            }

            for field_defs in input_object_type_extension.input_fields_definition() {
                for field_def in field_defs.input_value_definitions() {
                    input_object_type.fields.push(parse_field(FieldDefs::InputValueDef(field_def), config));
                }
            }
        }
    }
}