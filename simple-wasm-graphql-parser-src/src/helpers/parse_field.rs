use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use crate::prelude::{FieldData, FieldArgumentData, REGEX, either, parse_directive_instance, FieldDefs, SchemaData, Config, Logger};


pub fn parse_field(field_def: FieldDefs, config: &Config) -> FieldData {
    match field_def {
        FieldDefs::FieldDef(field_def) => {
            let mut field_name = field_def.name().unwrap().text().to_string();
            field_name = REGEX.match_whitespace.replace_all(&field_name, "").to_string();

            let field_description = match field_def.description() {
                Some(desc) => Some(REGEX.match_description_beginning_end_quotes.replace_all(&desc.to_string(), "").to_string()),
                None => None
            };
            let field_directives =  parse_directive_instance(field_def.directives(), config);

            // Field names are always correct, but field types have to be parsed manually
            let mut field_type = field_def.ty().unwrap().to_string();

            // Just match the type from the string, just in case
            match REGEX.match_complete_type.captures(&field_type).unwrap() {
                Some(caps) => field_type = caps.get(0).unwrap().as_str().to_string(),
                None => Logger::error("Error getting complete type for {}", Some(&field_type), config)
            }
            let field_complete_type = field_type.clone();

            // Now we get the actual type by removing [] and !
            field_type = REGEX.match_brackets_and_exclamation.replace_all(&field_type, "").to_string();

            // Now that we have the actual type, we must now check for some extra stuff regarding this current field, like if it is an enum or whatever
            let is_array = field_complete_type.contains("[");
            
            // Basically, in SDL, we can have something like [[[String]!]]!
            // So if we have more exclamation than brackets, then we have somehting like [[[String!]!]!]! which makes it a required field
            // If we have less exclamations than brackets, but we end with an exclamation, like [[[String]!]]!, this would also be a required field
            // Any other case means the field is optional
            let exclamations: usize = REGEX.match_exclamations.find_iter(&field_complete_type).count();

            let closing_brackets = REGEX.match_closing_brackets.find_iter(&field_complete_type).count();
            
            let is_nullable = either!(is_array => either!(exclamations > closing_brackets => false; either!(field_complete_type.ends_with("]!") => false; true)); either!(exclamations > 0 => false; true));

            // Now we iterate arguments in this field if it has any
            let mut arguments: Vec<FieldArgumentData> = vec![];

            for argument_def in field_def.arguments_definition() {

                for (index, input_value_def) in argument_def.input_value_definitions().enumerate() {
                    let argument_name = input_value_def.name().unwrap().to_string();
                    field_name = REGEX.match_whitespace.replace_all(&field_name, "").to_string();

                    let mut argument_complete_type = input_value_def.ty().unwrap().to_string();                     

                    let mut argument_type = REGEX.match_brackets_and_exclamation.replace_all(&argument_complete_type, "").to_string();
                    match REGEX.match_word.captures(&argument_complete_type).unwrap() {
                        Some(caps) => argument_type = caps.get(0).unwrap().as_str().to_string(),
                        None => Logger::error("Error getting type for {}", Some(&argument_complete_type), config)
                    }
                    
                    // Just match the type from the string, just in case
                    match REGEX.match_complete_type.captures(&argument_complete_type).unwrap() {
                        Some(caps) => argument_complete_type = caps.get(0).unwrap().as_str().to_string(),
                        None => Logger::error("Error getting complete type for {}", Some(&field_type), config)
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

            let field = FieldData {
                name: field_name,
                field_type: field_type,
                field_complete_type: field_complete_type,
                description: field_description,
                directives: field_directives,
                default_value: None, // Does't have the ability of setting default values at end of line
                arguments: either!(arguments.len() != 0 => Some(arguments); None),
                is_nullable,
                is_array,
                is_enum: false,
                is_scalar: false,
                is_union: false
            };

            return field;
        },
        FieldDefs::InputValueDef(field_def) => {
            let mut field_name = field_def.name().unwrap().text().to_string();
            field_name = REGEX.match_whitespace.replace_all(&field_name, "").to_string();


            let field_description = match field_def.description() {
                Some(desc) => Some(REGEX.match_description_beginning_end_quotes.replace_all(&desc.to_string(), "").to_string()),
                None => None
            };
            let field_directives =  parse_directive_instance(field_def.directives(), config);

            let mut field_type = field_def.ty().unwrap().to_string();

            if field_type.contains("#") {
                field_type = REGEX.match_comments_before_type.replace_all(&field_type, "").to_string();
            }

            match REGEX.match_complete_type.captures(&field_type).unwrap() {
                Some(caps) => field_type = caps.get(0).unwrap().as_str().to_string(),
                None => Logger::error("Error getting complete type for {}", Some(&field_type), config)
            }
            let field_complete_type = field_type.clone();

            field_type = REGEX.match_brackets_and_exclamation.replace_all(&field_type, "").to_string();

            let is_array = field_complete_type.contains("[");
            let exclamations: usize = REGEX.match_exclamations.find_iter(&field_complete_type).count();
            let closing_brackets = REGEX.match_closing_brackets.find_iter(&field_complete_type).count();
            let is_nullable = either!(is_array => either!(exclamations > closing_brackets => false; either!(field_complete_type.ends_with("]!") => false; true)); either!(exclamations > 0 => false; true));

            let default_value =  match field_def.default_value() {
                Some(val) => {
                    let default_value = REGEX.match_word.captures(&val.to_string()).unwrap().unwrap()[0].to_string();
                    Some(default_value)
                },
                None => None
            };

            let field = FieldData {
                name: field_name,
                field_type: field_type,
                field_complete_type: field_complete_type,
                description: field_description,
                directives: field_directives,
                default_value: default_value,
                arguments: None,
                is_nullable,
                is_array,
                is_enum: false,
                is_scalar: false,
                is_union: false
            };

            return field;
        },
        FieldDefs::VariableDef(field_def) => {
            let mut field_name = field_def.to_string();
            field_name = REGEX.match_whitespace.replace_all(&field_name, "").to_string();

            let field_description = None;

            let field_directives =  parse_directive_instance(field_def.directives(), config);

            let mut field_type = field_def.ty().unwrap().to_string();

            if field_type.contains("#") {
                field_type = REGEX.match_comments_before_type.replace_all(&field_type, "").to_string();
            }

            match REGEX.match_complete_type.captures(&field_type).unwrap() {
                Some(caps) => field_type = caps.get(0).unwrap().as_str().to_string(),
                None => Logger::error("Error getting complete type for {}", Some(&field_type), config)
            }
            let field_complete_type = field_type.clone();

            field_type = REGEX.match_brackets_and_exclamation.replace_all(&field_type, "").to_string();

            let is_array = field_complete_type.contains("[");
            
            let exclamations: usize = REGEX.match_exclamations.find_iter(&field_complete_type).count();

            let closing_brackets = REGEX.match_closing_brackets.find_iter(&field_complete_type).count();
            
            let is_nullable = either!(is_array => either!(exclamations > closing_brackets => false; either!(field_complete_type.ends_with("]!") => false; true)); either!(exclamations > 0 => false; true));

            let default_value =  match field_def.default_value() {
                Some(val) => {
                    let default_value = REGEX.match_word.captures(&val.to_string()).unwrap().unwrap()[0].to_string();
                    Some(default_value)
                },
                None => None
            };

            let field = FieldData {
                name: field_name,
                field_type: field_type,
                field_complete_type: field_complete_type,
                description: field_description,
                directives: field_directives,
                arguments: None,
                default_value: default_value,
                is_nullable,
                is_array,
                is_enum: false,
                is_scalar: false,
                is_union: false
            };

            return field;
        }
    }
}


// Process every field
pub fn process_fields(schema_data: &mut SchemaData, _config: &Config) {
    let enums: Vec<String> = if schema_data.enum_types.is_some() { schema_data.enum_types.as_ref().unwrap().iter().map(|e| e.name.clone()).collect() } else { vec![] };
    let scalars: Vec<String> = if schema_data.scalar_types.is_some() { schema_data.scalar_types.as_ref().unwrap().iter().map(|s| s.name.clone()).collect() } else { vec![] };
    let unions: Vec<String> = if schema_data.union_types.is_some() { schema_data.union_types.as_ref().unwrap().iter().map(|s| s.name.clone()).collect() } else { vec![] };

    schema_data.input_object_types.iter_mut().flatten().for_each(|input_object_type| {
        input_object_type.fields.iter_mut().for_each(|field| {
            if enums.iter().any(|e| *e == field.field_type) { field.is_enum = true; }
            else if scalars.iter().any(|s| *s == field.field_type) { field.is_scalar = true; }
            else if unions.iter().any(|u| *u == field.field_type) { field.is_union = true; }

            field.arguments.iter_mut().flatten().for_each(|argument| {
                if enums.iter().any(|e| *e == argument.argument_type) { argument.is_enum = true; }
                else if scalars.iter().any(|s| *s == argument.argument_type) { argument.is_scalar = true; }
                else if unions.iter().any(|u| *u == argument.argument_type) { argument.is_union = true; }
            })
        })
    });

    schema_data.interface_types.iter_mut().flatten().for_each(|interface_type| {
        interface_type.fields.iter_mut().for_each(|field| {
            if enums.iter().any(|e| *e == field.field_type) { field.is_enum = true; }
            else if scalars.iter().any(|s| *s == field.field_type) { field.is_scalar = true; }
            else if unions.iter().any(|u| *u == field.field_type) { field.is_union = true; }

            field.arguments.iter_mut().flatten().for_each(|argument| {
                if enums.iter().any(|e| *e == argument.argument_type) { argument.is_enum = true; }
                else if scalars.iter().any(|s| *s == argument.argument_type) { argument.is_scalar = true; }
                else if unions.iter().any(|u| *u == argument.argument_type) { argument.is_union = true; }
            })
        })
    });
    
    schema_data.object_types.iter_mut().flatten().for_each(|object_type| {
        object_type.fields.iter_mut().for_each(|field| {
            if enums.iter().any(|e| *e == field.field_type) { field.is_enum = true; }
            else if scalars.iter().any(|s| *s == field.field_type) { field.is_scalar = true; }
            else if unions.iter().any(|u| *u == field.field_type) { field.is_union = true; }

            field.arguments.iter_mut().flatten().for_each(|argument| {
                if enums.iter().any(|e| *e == argument.argument_type) { argument.is_enum = true; }
                else if scalars.iter().any(|s| *s == argument.argument_type) { argument.is_scalar = true; }
                else if unions.iter().any(|u| *u == argument.argument_type) { argument.is_union = true; }
            })
        })
    });

    schema_data.operation_types.iter_mut().flatten().for_each(|operation_type| {
        operation_type.fields.iter_mut().for_each(|field| {
            if enums.iter().any(|e| *e == field.field_type) { field.is_enum = true; }
            else if scalars.iter().any(|s| *s == field.field_type) { field.is_scalar = true; }
            else if unions.iter().any(|u| *u == field.field_type) { field.is_union = true; }

            field.arguments.iter_mut().flatten().for_each(|argument| {
                if enums.iter().any(|e| *e == argument.argument_type) { argument.is_enum = true; }
                else if scalars.iter().any(|s| *s == argument.argument_type) { argument.is_scalar = true; }
                else if unions.iter().any(|u| *u == argument.argument_type) { argument.is_union = true; }
            })
        })
    });

    // if let Some(ref mut input_object_types) = schema_data.input_object_types {
    //     for input_object_type in input_object_types {
    //         let ref mut fields = input_object_type.fields;

    //         for field in fields {
    //             if enums.par_iter().any(|e| *e == field.field_type) { field.is_scalar = true; }
    //             else if scalars.par_iter().any(|s| *s == field.field_type) { field.is_scalar = true; }

    //             if enums.iter().any(|e| &field.name == e) {
    //                 field.is_enum = Some(true);
    //             }   
    //             else if scalars.iter().any(|e| &field.name == e) {
    //                 field.is_scalar = Some(true);
    //             }

    //             if let Some(ref mut arguments) = field.arguments {
    //                 for argument in arguments {
    //                     argument.is_scalar = Some(false);
    //                     argument.is_enum = Some(false);
    //                     if enums.iter().any(|e| &argument.name == e) {
    //                         argument.is_scalar = Some(false);
    //                         argument.is_enum = Some(true);
    //                     }   
    //                     else if scalars.iter().any(|e| &argument.name == e) {
    //                         argument.is_enum = Some(false);
    //                         argument.is_scalar = Some(true);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // if let Some(ref mut interface_types) = schema_data.interface_types {
    //     for interface_type in interface_types {
    //         let ref mut fields = interface_type.fields;
            
    //         for field in fields {
    //             field.is_scalar = Some(false);
    //             field.is_enum = Some(false);

    //             if enums.iter().any(|e| &field.name == e) {
    //                 field.is_enum = Some(true);
    //             }   
    //             else if scalars.iter().any(|e| &field.name == e) {
    //                 field.is_scalar = Some(true);
    //             }

    //             if let Some(ref mut arguments) = field.arguments {
    //                 for argument in arguments {
    //                     argument.is_scalar = Some(false);
    //                     argument.is_enum = Some(false);
    //                     if enums.iter().any(|e| &argument.name == e) {
    //                         argument.is_scalar = Some(false);
    //                         argument.is_enum = Some(true);
    //                     }   
    //                     else if scalars.iter().any(|e| &argument.name == e) {
    //                         argument.is_enum = Some(false);
    //                         argument.is_scalar = Some(true);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // if let Some(ref mut object_types) = schema_data.object_types {
    //     for object_type in object_types {
    //         let ref mut fields = object_type.fields;
            
    //         for field in fields {
    //             field.is_scalar = Some(false);
    //             field.is_enum = Some(false);

    //             if enums.iter().any(|e| &field.name == e) {
    //                 field.is_enum = Some(true);
    //             }   
    //             else if scalars.iter().any(|e| &field.name == e) {
    //                 field.is_scalar = Some(true);
    //             }

    //             if let Some(ref mut arguments) = field.arguments {
    //                 for argument in arguments {
    //                     argument.is_scalar = Some(false);
    //                     argument.is_enum = Some(false);
    //                     if enums.iter().any(|e| &argument.name == e) {
    //                         argument.is_scalar = Some(false);
    //                         argument.is_enum = Some(true);
    //                     }   
    //                     else if scalars.iter().any(|e| &argument.name == e) {
    //                         argument.is_enum = Some(false);
    //                         argument.is_scalar = Some(true);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // if let Some(ref mut operation_types) = schema_data.operation_types {
    //     for operation_type in operation_types {
    //         let ref mut fields = operation_type.fields;
            
    //         for field in fields {
    //             field.is_scalar = Some(false);
    //             field.is_enum = Some(false);

    //             if enums.iter().any(|e| &field.name == e) {
    //                 field.is_enum = Some(true);
    //             }   
    //             else if scalars.iter().any(|e| &field.name == e) {
    //                 field.is_scalar = Some(true);
    //             }

    //             if let Some(ref mut arguments) = field.arguments {
    //                 for argument in arguments {
    //                     argument.is_scalar = Some(false);
    //                     argument.is_enum = Some(false);
    //                     if enums.iter().any(|e| &argument.name == e) {
    //                         argument.is_scalar = Some(false);
    //                         argument.is_enum = Some(true);
    //                     }   
    //                     else if scalars.iter().any(|e| &argument.name == e) {
    //                         argument.is_enum = Some(false);
    //                         argument.is_scalar = Some(true);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    // if let Some(ref mut operation_types) = schema_data.operation_types {
    //     for operation_type in operation_types {
    //         let ref mut fields = operation_type.fields;
            
    //         for field in fields {
    //             field.is_scalar = Some(false);
    //             field.is_enum = Some(false);

    //             if enums.iter().any(|e| &field.name == e) {
    //                 field.is_enum = Some(true);
    //             }   
    //             else if scalars.iter().any(|e| &field.name == e) {
    //                 field.is_scalar = Some(true);
    //             }
                

    //             if let Some(ref mut arguments) = field.arguments {
    //                 for argument in arguments {
    //                     argument.is_scalar = Some(false);
    //                     argument.is_enum = Some(false);
    //                     if enums.iter().any(|e| &argument.name == e) {
    //                         argument.is_scalar = Some(false);
    //                         argument.is_enum = Some(true);
    //                     }   
    //                     else if scalars.iter().any(|e| &argument.name == e) {
    //                         argument.is_enum = Some(false);
    //                         argument.is_scalar = Some(true);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}


pub fn par_process_fields(schema_data: &mut SchemaData, _config: &Config) {
    let enums: Vec<String> = if schema_data.enum_types.is_some() { schema_data.enum_types.as_ref().unwrap().par_iter().map(|e| e.name.clone()).collect() } else { vec![] };
    let scalars: Vec<String> = if schema_data.scalar_types.is_some() { schema_data.scalar_types.as_ref().unwrap().par_iter().map(|s| s.name.clone()).collect() } else { vec![] };
    let unions: Vec<String> = if schema_data.union_types.is_some() { schema_data.union_types.as_ref().unwrap().par_iter().map(|u| u.name.clone()).collect() } else { vec![] };

    schema_data.input_object_types.par_iter_mut().flatten().for_each(|input_object_type| {
        input_object_type.fields.par_iter_mut().for_each(|field| {
            if enums.par_iter().any(|e| *e == field.field_type) { field.is_enum = true; }
            else if scalars.par_iter().any(|s| *s == field.field_type) { field.is_scalar = true; }
            else if unions.par_iter().any(|u| *u == field.field_type) { field.is_union = true; }

            field.arguments.par_iter_mut().flatten().for_each(|argument| {
                if enums.par_iter().any(|e| *e == argument.argument_type) { argument.is_enum = true; }
                else if scalars.par_iter().any(|s| *s == argument.argument_type) { argument.is_scalar = true; }
                else if unions.par_iter().any(|u| *u == argument.argument_type) { argument.is_union = true; }
            })
        })
    });

    schema_data.interface_types.par_iter_mut().flatten().for_each(|interface_type| {
        interface_type.fields.par_iter_mut().for_each(|field| {
            if enums.par_iter().any(|e| *e == field.field_type) { field.is_enum = true; }
            else if scalars.par_iter().any(|s| *s == field.field_type) { field.is_scalar = true; }
            else if unions.par_iter().any(|u| *u == field.field_type) { field.is_union = true; }

            field.arguments.par_iter_mut().flatten().for_each(|argument| {
                if enums.par_iter().any(|e| *e == argument.argument_type) { argument.is_enum = true; }
                else if scalars.par_iter().any(|s| *s == argument.argument_type) { argument.is_scalar = true; }
                else if unions.par_iter().any(|u| *u == argument.argument_type) { argument.is_union = true; }
            })
        })
    });
    
    schema_data.object_types.par_iter_mut().flatten().for_each(|object_type| {
        object_type.fields.par_iter_mut().for_each(|field| {
            if enums.par_iter().any(|e| *e == field.field_type) { field.is_enum = true; }
            else if scalars.par_iter().any(|s| *s == field.field_type) { field.is_scalar = true; }
            else if unions.par_iter().any(|u| *u == field.field_type) { field.is_union = true; }

            field.arguments.par_iter_mut().flatten().for_each(|argument| {
                if enums.par_iter().any(|e| *e == argument.argument_type) { argument.is_enum = true; }
                else if scalars.par_iter().any(|s| *s == argument.argument_type) { argument.is_scalar = true; }
                else if unions.par_iter().any(|u| *u == argument.argument_type) { argument.is_union = true; }
            })
        })
    });

    schema_data.operation_types.par_iter_mut().flatten().for_each(|operation_type| {
        operation_type.fields.par_iter_mut().for_each(|field| {
            if enums.par_iter().any(|e| *e == field.field_type) { field.is_enum = true; }
            else if scalars.par_iter().any(|s| *s == field.field_type) { field.is_scalar = true; }
            else if unions.par_iter().any(|u| *u == field.field_type) { field.is_union = true; }

            field.arguments.par_iter_mut().flatten().for_each(|argument| {
                if enums.par_iter().any(|e| *e == argument.argument_type) { argument.is_enum = true; }
                else if scalars.par_iter().any(|s| *s == argument.argument_type) { argument.is_scalar = true; }
                else if unions.par_iter().any(|u| *u == argument.argument_type) { argument.is_union = true; }
            })
        })
    });
}
