use std::str;
use apollo_parser::ast::{Directives, DirectiveDefinition};
use crate::prelude::{DirectiveData, DirectiveArgumentValues, REGEX, either, Directive, FieldArgumentData, Config, Logger};

struct TempArgumentData {
    start_position: usize
}

struct TempArgumentStringValueData {
    start_position: usize,
    end_position: usize
}

// Used to parse directive definitions, which are just the directive declarations
pub fn parse_directive_def(directive_def: DirectiveDefinition, config: &Config) -> Directive {
    let mut name = directive_def.name().unwrap().text().to_string();
    name = REGEX.match_whitespace.replace_all(&name, "").to_string();

    let description = match directive_def.description() {
        Some(desc) => Some(REGEX.match_description_beginning_end_quotes.replace_all(&desc.to_string(), "").to_string()),
        None => None,
    };

    let mut arguments: Vec<FieldArgumentData> = vec![];

    for argument_def in directive_def.arguments_definition() {

        for (index, input_value_def) in argument_def.input_value_definitions().enumerate() {
            let argument_name = input_value_def.name().unwrap().to_string();
            let mut argument_complete_type = input_value_def.ty().unwrap().to_string(); 
            let argument_type = REGEX.match_brackets_and_exclamation.replace_all(&argument_complete_type, "").to_string();
            
            // Just match the type from the string, just in case
            match REGEX.match_complete_type.captures(&argument_complete_type).unwrap() {
                Some(caps) => argument_complete_type = caps.get(0).unwrap().as_str().to_string(),
                None => Logger::error("Error getting complete type for", Some(&argument_complete_type), config)
            }
            
            let input_value_description = match input_value_def.description() {
                Some(desc) => Some(REGEX.match_description_beginning_end_quotes.replace_all(&desc.to_string(), "").to_string()),
                None => None,
            };
            
            let input_value_directives = parse_directive_instance(input_value_def.directives(), config);
            
            let input_value_default_value =  match input_value_def.default_value() {
                Some(val) => {
                    let default_value = REGEX.match_word.captures(&val.to_string()).unwrap().unwrap()[0].to_string();
                    Some(default_value)
                },
                None => None
            };

            let is_array = argument_complete_type.contains("[");
            let exclamations: usize = REGEX.match_exclamations.find_iter(&argument_complete_type).count();
            let closing_brackets = REGEX.match_closing_brackets.find_iter(&argument_complete_type).count();
            let is_nullable = either!(is_array => either!(exclamations > closing_brackets => false; either!(argument_complete_type.ends_with("]!") => false; true)); either!(exclamations > 0 => false; true));


            let field_argument_data = FieldArgumentData {
                argument_index: index,
                name: argument_name,
                argument_type: argument_type,
                argument_complete_type: argument_complete_type,
                default_value: input_value_default_value,
                description: input_value_description,
                directives: input_value_directives,
                is_array,
                is_nullable,
                is_enum: false,
                is_scalar: false,
                is_union: false
            };

            arguments.push(field_argument_data);
        }
    }

    let mut locations: Vec<String> = vec![];


    for location in directive_def.directive_locations().unwrap().directive_locations() {
        let mut loc = location.to_string();
        loc = REGEX.match_whitespace.replace_all(&loc, "").to_string();
        locations.push(loc);
    }

    let repeateable_token: bool = if directive_def.repeatable_token().is_some() { true } else { false };
    
    
    let directive_type: Directive = Directive {
        name: name,
        description: description,
        arguments: either!(arguments.len() != 0 => Some(arguments); None),
        locations: locations,
        repeatable: repeateable_token,
    };

    return directive_type;
}


// Used to parse directive calls in fields and whatnot
pub fn parse_directive_instance(to_parse: Option<Directives>, config: &Config) -> Option<Vec<DirectiveData>> {
    return match to_parse {
        Some(directives) => {
            let raw_directives = directives.to_string();
            let mut directives_data: Vec<DirectiveData> = vec![];

            let directives_with_parenthesis_caps = REGEX.match_all_directives_with_parenthesis.find_iter(&raw_directives);

            for cap in directives_with_parenthesis_caps {
                let directive_cap = cap.unwrap().as_str();
                let mut name = REGEX.match_directive_name.captures(directive_cap).unwrap().unwrap()[0].to_string();
                name = str::replace(&name, "@", "");
                let data = REGEX.match_directive_name.replace_all(directive_cap, "").to_string();

                let directive_data = DirectiveData {
                    name: name,
                    values: parse_directive_data(data, config)
                };

                directives_data.push(directive_data);
            }

            let directives_without_parenthesis = REGEX.match_all_directives_with_parenthesis.replace_all(&raw_directives, "").to_string();

            let directives_without_parenthesis_caps = REGEX.match_directive_name.find_iter(&directives_without_parenthesis);

            for cap in directives_without_parenthesis_caps {
                let directive_cap = cap.unwrap().as_str();

                let name = str::replace(directive_cap, "@", "");

                let directive_data = DirectiveData {
                    name: name,
                    values: None
                };

                directives_data.push(directive_data);
            }

            either!(directives_data.len() > 0 => Some(directives_data); None)
        },
        None => None
    };
}

pub fn parse_directive_data(to_parse: String, _config: &Config) -> Option<Vec<DirectiveArgumentValues>> {
    let mut directive_values: Vec<DirectiveArgumentValues> = vec![];
    let data = REGEX.match_all_parenthesis.replace_all(&to_parse, "").to_string();

    let argument_caps = REGEX.match_field_name.find_iter(&data);
    

    // Iterate ALL values with multiline strings since these can be recursive and have fields within
    let mut arguments_multiline_string_values: Vec<TempArgumentStringValueData> = vec![];
    let arguments_multiline_string_values_cap = REGEX.global_match_description_with_quotes.find_iter(&data);

    for cap in arguments_multiline_string_values_cap {
        if cap.is_ok() {
            arguments_multiline_string_values.push(TempArgumentStringValueData {
                start_position: cap.as_ref().unwrap().start(),
                end_position: cap.as_ref().unwrap().end()
            });
        }
    }

    // Now we iterate all single line strings. These can't have quotes inside them, unless we escape them, but we won't deal with that bs
    // Single line strings can have fields within too like @exmaple("id: 1")
    let mut arguments_singleline_string_values: Vec<TempArgumentStringValueData> = vec![];
    let arguments_singleline_string_values_caps = REGEX.match_single_quote_description.find_iter(&data);

    for cap in arguments_singleline_string_values_caps {
        if cap.is_ok() {
            let start_position = cap.as_ref().unwrap().start();
            let end_position = cap.as_ref().unwrap().end();
            let mut is_inside_multiline_string = false;

            for multiline_values in &arguments_multiline_string_values {
                // This is the only case we should handle since all other cases means a wrongly formatted schema
                if start_position > multiline_values.start_position && start_position < multiline_values.end_position {
                    is_inside_multiline_string = true;
                    break;
                }
            }

            if !is_inside_multiline_string {
                arguments_singleline_string_values.push(TempArgumentStringValueData {
                    start_position: start_position,
                    end_position: end_position
                });
            }
        }
    }
    
    let mut arguments: Vec<TempArgumentData> = vec![];
    let arguments_caps = REGEX.match_field_name.find_iter(&data);
    
    // Now that we know where all strings are, we know all fields that belong inside strings, and therefor know exacltly what are arguments and which are not
    for cap in arguments_caps {
        if cap.is_ok() {
            let start_position = cap.as_ref().unwrap().start();
            let mut is_inside_string = false;

            for multiline_values in &arguments_multiline_string_values {
                // This is the only case we should handle since all other cases means a wrongly formatted schema
                if start_position > multiline_values.start_position && start_position < multiline_values.end_position {
                    is_inside_string = true;
                    break;
                }
            }
            if is_inside_string {
                continue
            }
            for singleline_values in &arguments_singleline_string_values {
                // This is the only case we should handle since all other cases means a wrongly formatted schema
                if start_position > singleline_values.start_position && start_position < singleline_values.end_position {
                    is_inside_string = true;
                    break;
                }
            }
            if is_inside_string {
                continue
            }

            arguments.push(TempArgumentData {
                start_position: start_position
            })
        }
    }

    // Now we know which fields are fields, so we can do this
    for cap in argument_caps {
        let argument = cap.as_ref().unwrap().as_str();
        let argument_end_position = cap.as_ref().unwrap().end();
        let mut next_argument_start_position: usize = data.len();


        // Iterate arguments and get start posiiton of next field
        for arg in &arguments {
            if arg.start_position > argument_end_position {
                // If this doesn't check out, then next_argument_start_position remains as the last index, which is ok since it means this argument is the last one
                if arg.start_position < next_argument_start_position {
                    next_argument_start_position = arg.start_position;
                }
            }
        }


        let argument_value = data.get(argument_end_position..next_argument_start_position).and_then(|raw_string| {
            // Now we must parse this raw string
            // We just have to remove everything prepending the start of the argument value
            let value_cap = REGEX.match_directive_argument_value.captures(raw_string);
            
            if value_cap.is_ok() {
                let caps = REGEX.match_directive_argument_value.captures(raw_string);
                if caps.is_ok() {
                    let unwrapped_caps = caps.unwrap();
                    if unwrapped_caps.is_some() {
                        return Some(unwrapped_caps.unwrap()[0].to_string());
                    }
                }
            }
            // return "" if there was some problem
            return Some("\"\"".to_string());
        });

        directive_values.push(DirectiveArgumentValues {
            parameter: argument.to_string(),
            value: argument_value.unwrap()
        })
    }

    either!(directive_values.len() > 0 => Some(directive_values); None)
}