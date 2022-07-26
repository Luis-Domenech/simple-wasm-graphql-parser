use apollo_parser::ast::{ScalarTypeDefinition, ScalarTypeExtension};
use crate::prelude::{ScalarType, DirectiveData, REGEX, parse_directive_instance, Config};

pub fn parse_scalar_type_def(scalar_type_def: ScalarTypeDefinition, config: &Config) -> ScalarType {
    let mut name = scalar_type_def.name().unwrap().text().to_string();
    name = REGEX.match_whitespace.replace_all(&name, "").to_string();

    let description = match scalar_type_def.description() {
        Some(desc) => Some(REGEX.match_description_beginning_end_quotes.replace_all(&desc.to_string(), "").to_string()),
        None => None,
    };

    let directives: Option<Vec<DirectiveData>> = parse_directive_instance(scalar_type_def.directives(), config);

    let scalar_type: ScalarType = ScalarType {
        name: name,
        description: description,
        directives: directives
    };

    return scalar_type;
}

pub fn parse_scalar_type_extension(scalar_type_extension: ScalarTypeExtension, scalar_types: &mut Vec<ScalarType>, config: &Config) {
    let mut extends = scalar_type_extension.name().unwrap().to_string();
    extends = REGEX.match_whitespace.replace_all(&extends, "").to_string();

    // If no match is found, then we simply do nothing this this extension. We won't return an error
    for scalar_type in scalar_types {
        if extends == scalar_type.name {
            match parse_directive_instance(scalar_type_extension.directives(), config) {
                Some(directives) => {
                    for directive in directives {
                        if let Some(ref mut d) = scalar_type.directives {
                            d.push(directive);
                        }
                        else {
                            scalar_type.directives = Some(vec![directive]);
                        }
                    }
                },
                None => {}
            }
        }
    }
}