use std::collections::HashMap;
use std::str;
use apollo_parser::ast::{EnumTypeDefinition, EnumTypeExtension, EnumValuesDefinition};
use crate::prelude::{EnumType, DirectiveData, EnumValueData, REGEX, either, parse_directive_instance, parse_directive_data, Config};


#[derive(Serialize, Deserialize, Debug)]
struct TempEnumValueDirectiveData {
    name: String,
    data: String
}

#[derive(Serialize, Deserialize, Debug)]
struct TempEnumValueData<'a> {
    value: &'a str,
    description: String,
    directives: Vec<TempEnumValueDirectiveData>
}

pub fn parse_enum_type_def(enum_type_def: EnumTypeDefinition, config: &Config) -> EnumType {
    let mut name = enum_type_def.name().unwrap().text().to_string();
    name = REGEX.match_whitespace.replace_all(&name, "").to_string();

    let description = match enum_type_def.description() {
        Some(desc) => Some(REGEX.match_description_beginning_end_quotes.replace_all(&desc.to_string(), "").to_string()),
        None => None,
    };

    let directives: Option<Vec<DirectiveData>> = parse_directive_instance(enum_type_def.directives(), config);
    let mut values: Vec<EnumValueData> = vec![];

    // Apollo Parser doesn't parse enum values correctly, so we do it ourselves
    // We'll do a lot of regex stuff here to get as much info as possible from eveything enclosed in an enum code block
    for value in enum_type_def.enum_values_definition() {
        for enum_value_data in parse_enum_values(value, config) {
            values.push(enum_value_data);
        }
    }


    let enum_type: EnumType = EnumType {
        name: name,
        description: description,
        directives: directives,
        values: values
    };

    return enum_type;
}

pub fn parse_enum_type_extension(enum_type_extension: EnumTypeExtension, enum_types: &mut Vec<EnumType>, config: &Config) {
    let mut extends = enum_type_extension.name().unwrap().to_string();
    extends = REGEX.match_whitespace.replace_all(&extends, "").to_string();

    // If no match is found, then we simply do nothing this this extension. We won't return an error
    for enum_type in enum_types {
        if extends == enum_type.name {
            match parse_directive_instance(enum_type_extension.directives(), config) {
                Some(directives) => {
                    for directive in directives {
                        if let Some(ref mut d) = enum_type.directives {
                            d.push(directive);
                        }
                        else {
                            enum_type.directives = Some(vec![directive]);
                        }
                    }
                },
                None => {}
            }

            for value in enum_type_extension.enum_values_definition() {
                for enum_value_data in parse_enum_values(value, config) {
                    enum_type.values.push(enum_value_data);
                }
            }
        }
    }
}

fn parse_enum_values(value: EnumValuesDefinition, config: &Config) -> Vec<EnumValueData> {
    let raw_string = value.to_string().clone();
    let mut values: Vec<EnumValueData> = vec![];
        
    // First, get enum values by removing everything else
    // Also, order matters as directives can have strings like descriptions, so we must remove all directives first and then we remove descriptions
    let mut raw_values = REGEX.match_all_directives_with_parenthesis.replace_all(&raw_string, "").to_string();
    raw_values = REGEX.match_directive_name.replace_all(&raw_values, "").to_string();
    raw_values = REGEX.global_match_description_with_quotes.replace_all(&raw_values, "").to_string();
    raw_values = REGEX.match_single_quote_description.replace_all(&raw_values, "").to_string();
    

    // Now we should have just have the actual enum values left as words, so capture them
    let mut vals: HashMap<String, TempEnumValueData> = HashMap::new();
    let vals_caps = REGEX.match_first_word.find_iter(&raw_values);

    for cap in vals_caps {  
        if cap.is_ok() {
            let v = cap.unwrap().as_str();

            vals.insert(v.to_string(), TempEnumValueData {
                value: v,
                description: "".to_string(),
                directives: vec![]
            });
        }
    }


    // Now that we have the values, we can determine where descriptions and directives belong to
    // Descriptions should ALWAYS go on top of their fields, so handle them first
    let mut vals_without_directives = REGEX.match_all_directives_with_parenthesis.replace_all(&raw_string, "").to_string();
    vals_without_directives = REGEX.match_directive_name.replace_all(&vals_without_directives, "").to_string();
    let mut descs: Vec<String> = vec![];

    let three_quotes_descriptions_caps = REGEX.global_match_description_with_quotes.find_iter(&vals_without_directives);

    for cap in three_quotes_descriptions_caps {  
        if cap.is_ok() {
            let mut description = cap.as_ref().unwrap().as_str().to_string();
            // Remove " from strings
            description = REGEX.match_description_beginning_end_quotes.replace_all(&description, "").to_string();
            
            let end_position = cap.as_ref().unwrap().end();

            descs.push(description.clone());

            // This will certainly get us our linked desc
            // let mut iter = vals_without_directives.char_indices();
            let v = vals_without_directives.get(end_position..).and_then(|s| {
                for cap in REGEX.match_first_word.find_iter(s) {
                    return Some(cap.unwrap().as_str().to_string().clone())
                }
                return None
            });

            if v.is_none() {
                panic!("Error getting enum value for description while parsing enum values");
            }

            let key = v.unwrap();

            // Chekc if v is avtually a value
            if vals.get(&key).is_none() {
                panic!("Supposed parsed enum value is not actually an enum value");
            }

            //not insert, but update
            vals.insert(key.to_string(), TempEnumValueData {
                value: vals[&key].value,
                description: description,
                directives: vec![]
            });
        }
    }

    // What a bad name right xD
    let vals_without_directives_and_three_quote_descs = REGEX.global_match_description_with_quotes.replace_all(&vals_without_directives, "").to_string();
    let single_quote_descriptions_caps = REGEX.match_single_quote_description.find_iter(&vals_without_directives_and_three_quote_descs);
    
    for cap in single_quote_descriptions_caps {  
        if cap.is_ok() {
            let mut description = cap.as_ref().unwrap().as_str().to_string();
            // Remove " from strings
            description = REGEX.match_description_beginning_end_quotes.replace_all(&description, "").to_string();
            
            let end_position = cap.as_ref().unwrap().end();

            descs.push(description.clone());

            // This will certainly get us our linked desc
            let v = vals_without_directives.get(end_position..).and_then(|s| {
                for cap in REGEX.match_first_word.find_iter(s) {
                    return Some(cap.unwrap().as_str().to_string().clone())
                }
                return None
            });

            if v.is_none() {
                panic!("Error getting enum value for description while parsing enum values");
            }

            let key = v.unwrap();

            // Chekc if v is avtually a value
            if vals.get(&key).is_none() {
                panic!("Supposed parsed enum value is not actually an enum value");
            }

            vals.insert(key.to_string(), TempEnumValueData {
                value: vals[&key].value,
                description: description,
                directives: vec![]
            });
        }
    }

    // Now that we have enum values and their descriptions, we can parse directives
    let mut vals_with_directives_only = raw_string.clone();

    for desc in descs {
        // Since we now have data on what descs are, we can remove them and not have to deal with my REGEX.global_match_description_with_quotes
        // which unfortunately matches strings inside directives with string values in parameters
        vals_with_directives_only = str::replace(&vals_with_directives_only, &desc, "");
    }

    let directives_with_parenthesis_caps = REGEX.match_all_directives_with_parenthesis.find_iter(&vals_with_directives_only);

    for cap in directives_with_parenthesis_caps {
        if cap.is_ok() {
            let directive_with_parenthesis_raw_string = cap.as_ref().unwrap().as_str();
            let directive_with_parenthesis_start_position = cap.as_ref().unwrap().start();
            let directive_with_parenthesis_end_position = cap.as_ref().unwrap().end();
            let directive_name_with_at = REGEX.match_directive_name.captures(&directive_with_parenthesis_raw_string).unwrap().unwrap()[0].to_string();
            let directive_name_only = str::replace(&directive_name_with_at, "@", "");
            let mut associated_enum_found = false;

            // Determine if directive is on same line of an enum value
            // Also, since directive name incldues @, we won't deal with issues of enum value having same name as directive
            for line in vals_with_directives_only.lines() {
                if line.contains(&directive_name_with_at) {

                    // Now we check if the directive is this line is the one we are currently parsing and if there are muyltiple, check each one
                    let directive_name_caps = REGEX.match_directive_name.find_iter(line);
                    let line_position_in_vals_with_directives_only = vals_with_directives_only.find(line).expect("Error getting line in vals_with_directives_only");                        
                    let mut is_currect_directive = false;

                    for cap in directive_name_caps {
                        if cap.is_ok() {
                            let cap_position_in_line = cap.as_ref().unwrap().start();
                            let cap_position_in_vals_with_directives_only = line_position_in_vals_with_directives_only + cap_position_in_line;
                            
                            // If this directive's start pos is the same as the directive we are parsing, we know this is the correct directive
                            if directive_with_parenthesis_start_position == cap_position_in_vals_with_directives_only {
                                is_currect_directive = true;
                                break;
                            }
                        }
                    }

                    if is_currect_directive {
                        // Check if line has enum, else it means the next enum we find is the enum associated to this directive
                        for v in vals.keys() {
                            // Before checking, remove ALL directives to avoid cases where enum value has same name as directive
                            let mut line_without_directives = REGEX.match_all_directives_with_parenthesis.replace_all(&line, "").to_string();
                            line_without_directives = REGEX.match_directive_name.replace_all(&line_without_directives, "").to_string();

                            if line_without_directives.contains(v) {
                                associated_enum_found = true;
                                
                                let data = str::replace(&directive_with_parenthesis_raw_string, &directive_name_with_at, "");

                                // TODO: Parse directive data and not just store it as a string
                                let directive_data = TempEnumValueDirectiveData {
                                    name: directive_name_only.to_string(),
                                    data: data,
                                };

                                let mut new_vec: Vec<TempEnumValueDirectiveData> = vec![];

                                // We have to do this in order to avoid borrow issues and lifetime issues and whatnot
                                for d in &vals[v].directives {
                                    new_vec.push(TempEnumValueDirectiveData {
                                        name: d.name.clone(),
                                        data: d.data.clone()
                                    });
                                }
                                new_vec.push(directive_data);

                                // We don't check if vals.directives has a directive with same name because directives can be repeatable
                                let temp = TempEnumValueData {
                                    value: vals[v].value,
                                    description: vals[v].description.clone(),
                                    directives: new_vec
                                };

                                vals.insert(v.to_string(), temp);

                                break;
                            }
                        }
                    }
                }
            }

            // If we have checked all lines and this current directive was not found, then it means it is a directive on top of a field, and thus run this logic then
            if !associated_enum_found {
                // Here we have the same issue that mutliple directives can be side to side, but now on top of a field
                // So the idea is to ge tthe next word rifght after this capture, but only after removing directives on the splice first
                let v = vals_with_directives_only.get(directive_with_parenthesis_end_position..).and_then(|s| {
                    let mut spliced_string = REGEX.match_all_directives_with_parenthesis.replace_all(s, "").to_string();
                    spliced_string = REGEX.match_directive_name.replace_all(&spliced_string, "").to_string();

                    for cap in REGEX.match_first_word.find_iter(&spliced_string) {
                        return Some(cap.unwrap().as_str().to_string().clone())
                    }
                    return None
                });

                if v.is_none() {
                    panic!("Error getting enum value for directives while parsing enum values");
                }

                let key = v.unwrap();

                // Check if v is actually an enum value
                if vals.get(&key).is_none() {
                    panic!("Supposed parsed enum value is not actually an enum value");
                }

                let data = str::replace(&directive_with_parenthesis_raw_string, &directive_name_with_at, "");

                // TODO: Parse directive data and not just store it as a string
                let directive_data = TempEnumValueDirectiveData {
                    name: directive_name_only.to_string(),
                    data: data,
                };

                let mut new_vec: Vec<TempEnumValueDirectiveData> = vec![];

                // We have to do this in order to avoid borrow issues and lifetime issues and whatnot
                for d in &vals[&key].directives {
                    new_vec.push(TempEnumValueDirectiveData {
                        name: d.name.clone(),
                        data: d.data.clone()
                    });
                }
                new_vec.push(directive_data);

                // We don't check if vals.directives has a directive with same name because directives can be repeatable
                let temp = TempEnumValueData {
                    value: vals[&key].value,
                    description: vals[&key].description.clone(),
                    directives: new_vec
                };

                vals.insert(key, temp);
            }
        }
    }

    // Now we have added directives data for all directives with parenthesis, now we repeat for the remaining non prenthesis directives
    let vals_with_directives_without_parenthesis_only = REGEX.match_all_directives_with_parenthesis.replace_all(&vals_with_directives_only, "").to_string();
    let directives_without_parenthesis_caps = REGEX.match_directive_name.find_iter(&vals_with_directives_without_parenthesis_only);

    for cap in directives_without_parenthesis_caps {
        if cap.is_ok() {
            let directive_name_with_at = cap.as_ref().unwrap().as_str().to_string();
            let directive_name_only = str::replace(directive_name_with_at.as_str(), "@", "");
            let directive_without_parenthesis_start_position = cap.as_ref().unwrap().start();
            let directive_without_parenthesis_end_position = cap.as_ref().unwrap().end();
            let mut associated_enum_found = false;

            for line in vals_with_directives_without_parenthesis_only.lines() {
                if line.contains(&directive_name_with_at) {
                    let directive_name_caps = REGEX.match_directive_name.find_iter(line);
                    let line_position_in_vals_with_directives_without_parenthesis_only = vals_with_directives_without_parenthesis_only.find(line).expect("Error getting line in vals_with_directives_without_parenthesis_only");                        
                    let mut is_currect_directive = false;

                    for cap in directive_name_caps {
                        if cap.is_ok() {
                            let cap_position_in_line = cap.as_ref().unwrap().start();
                            let cap_position_in_vals_with_directives_only = line_position_in_vals_with_directives_without_parenthesis_only + cap_position_in_line;
                            
                            if directive_without_parenthesis_start_position == cap_position_in_vals_with_directives_only {
                                is_currect_directive = true;
                                break;
                            }
                        }
                    }

                    if is_currect_directive {
                        for v in vals.keys() {
                            // Before checking, remove ALL directives to avoid cases where enum value has same name as directive
                            let line_without_directives = REGEX.match_directive_name.replace_all(&line, "").to_string();

                            if line_without_directives.contains(v) {
                                associated_enum_found = true;
                                // TODO: Parse directive data and not just store it as a string
                                let directive_data = TempEnumValueDirectiveData {
                                    name: directive_name_only.to_string(),
                                    data: "".to_string(),
                                };

                                let mut new_vec: Vec<TempEnumValueDirectiveData> = vec![];

                                // We have to do this in order to avoid borrow issues and lifetime issues and whatnot
                                for d in &vals[v].directives {
                                    new_vec.push(TempEnumValueDirectiveData {
                                        name: d.name.clone(),
                                        data: d.data.clone()
                                    });
                                }
                                new_vec.push(directive_data);

                                // We don't check if vals.directives has a directive with same name because directives can be repeatable
                                let temp = TempEnumValueData {
                                    value: vals[v].value,
                                    description: vals[v].description.clone(),
                                    directives: new_vec
                                };

                                vals.insert(v.to_string(), temp);

                                break;
                            }
                        }
                    }
                }
            }

            if !associated_enum_found {
                // here we use the same logic as before where we splice from capture's end position onwards and match the first word, only after remaining directives
                let v = vals_with_directives_without_parenthesis_only.get(directive_without_parenthesis_end_position..).and_then(|s| {
                    let spliced_string = REGEX.match_directive_name.replace_all(s, "").to_string();

                    for cap in REGEX.match_first_word.find_iter(&spliced_string) {
                        return Some(cap.unwrap().as_str().to_string().clone())
                    }
                    return None
                });

                if v.is_none() {
                    panic!("Error getting enum value for directives while parsing enum values");
                }

                let key = v.unwrap();

                // Check if v is actually an enum value
                if vals.get(&key).is_none() {
                    panic!("Supposed parsed enum value is not actually an enum value");
                }

                // TODO: Parse directive data and not just store it as a string
                let directive_data = TempEnumValueDirectiveData {
                    name: directive_name_only.to_string(),
                    data: "".to_string(),
                };

                let mut new_vec: Vec<TempEnumValueDirectiveData> = vec![];

                // We have to do this in order to avoid borrow issues and lifetime issues and whatnot
                for d in &vals[&key].directives {
                    new_vec.push(TempEnumValueDirectiveData {
                        name: d.name.clone(),
                        data: d.data.clone()
                    });
                }
                new_vec.push(directive_data);

                // We don't check if vals.directives has a directive with same name because directives can be repeatable
                let temp = TempEnumValueData {
                    value: vals[&key].value,
                    description: vals[&key].description.clone(),
                    directives: new_vec
                };

                vals.insert(key, temp);   
            }
        }
    }

    // Now we must iterate all of the data we have gathered for the enum values and convert them to our expected structs
    for enum_value in vals.values() {
        let mut new_vec: Vec<DirectiveData> = vec![];

        // We have to do this in order to avoid borrow issues and lifetime issues and whatnot
        for d in &enum_value.directives {
            new_vec.push(DirectiveData {
                name: d.name.clone(),
                values: parse_directive_data(d.data.clone(), config)
            });
        }


        let enum_value_data: EnumValueData = EnumValueData {
            value: enum_value.value.to_string(),
            description: either!(enum_value.description == "" => None; Some(enum_value.description.clone())),
            directives: either!(enum_value.directives.len() == 0 => None; Some(new_vec))
        };

        values.push(enum_value_data);
    }

    values
}