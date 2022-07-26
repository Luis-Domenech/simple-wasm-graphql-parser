use apollo_parser::ast::{InterfaceTypeDefinition, InterfaceTypeExtension};
use crate::prelude::{InterfaceType, DirectiveData, REGEX, parse_directive_instance, parse_field, FieldDefs, Config};

pub fn parse_interface_type_def(interface_type_def: InterfaceTypeDefinition, config: &Config) -> InterfaceType {
    let mut name = interface_type_def.name().unwrap().text().to_string();
    name = REGEX.match_whitespace.replace_all(&name, "").to_string();

    let description = match interface_type_def.description() {
        Some(desc) => Some(REGEX.match_description_beginning_end_quotes.replace_all(&desc.to_string(), "").to_string()),
        None => None,
    };


    let directives: Option<Vec<DirectiveData>> = parse_directive_instance(interface_type_def.directives(), config);

    let mut implements: Vec<String> = vec![];

    let mut interface_type = InterfaceType {
        name: name,
        fields: vec![],
        description: description,
        directives: directives,
        implements: None
    };

    for field_def in interface_type_def.fields_definition().unwrap().field_definitions() {
        interface_type.fields.push(parse_field(FieldDefs::FieldDef(field_def), config));
    }

    for implement in interface_type_def.implements_interfaces().iter() {
        // We will just store name of the interface being implemented. We don't need to store data we already have.
        for named_type in implement.named_types() {
            let implement_name = REGEX.match_word.captures(&named_type.name().unwrap().to_string()).unwrap().unwrap()[0].to_string();
            implements.push(implement_name);
        }
    }

    if implements.len() > 0 {
        interface_type.implements = Some(implements);
    }


    return interface_type;
}

// Here we iterate the object types we created and just directly append the info from the extension
pub fn parse_interface_type_extension(interface_type_extension: InterfaceTypeExtension, interface_types: &mut Vec<InterfaceType>, config: &Config) {
    let mut extends = interface_type_extension.name().unwrap().to_string();
    extends = REGEX.match_whitespace.replace_all(&extends, "").to_string();

    // If no match is found, then we simply do nothing this this extension. We won't return an error
    for interface_type in interface_types {
        if extends == interface_type.name {
            match parse_directive_instance(interface_type_extension.directives(), config) {
                Some(directives) => {
                    for directive in directives {
                        if let Some(ref mut d) = interface_type.directives {
                            d.push(directive);
                        }
                        else {
                            interface_type.directives = Some(vec![directive]);
                        }
                    }
                },
                None => {}
            }

            for field_def in interface_type_extension.fields_definition().unwrap().field_definitions() {
                interface_type.fields.push(parse_field(FieldDefs::FieldDef(field_def), config));
            }

            let implements: Option<String> = if interface_type_extension.implements_interfaces().is_some() { Some(interface_type_extension.implements_interfaces().unwrap().to_string()) } else { None };
            
            if implements.is_some() {
                if let Some(ref mut i) = interface_type.implements {
                    i.push(implements.unwrap());
                }
                else {
                    interface_type.implements = Some(vec![implements.unwrap()]);
                }
            }
        }
    }
}