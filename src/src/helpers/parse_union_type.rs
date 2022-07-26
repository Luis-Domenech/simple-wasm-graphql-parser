use apollo_parser::ast::{UnionTypeDefinition, UnionTypeExtension};
use crate::prelude::{UnionType, DirectiveData, REGEX, either, parse_directive_instance, Config};

pub fn parse_union_type_def(union_type_def: UnionTypeDefinition, config: &Config) -> UnionType {
    let mut name = union_type_def.name().unwrap().text().to_string();
    name = REGEX.match_whitespace.replace_all(&name, "").to_string();

    let description = match union_type_def.description() {
        Some(desc) => Some(REGEX.match_description_beginning_end_quotes.replace_all(&desc.to_string(), "").to_string()),
        None => None,
    };

    let directives: Option<Vec<DirectiveData>> = parse_directive_instance(union_type_def.directives(), config);
    let mut member_types: Vec<String> = vec![];

    // Apollo parser doesn't parse unions correctly, so we have to do it manually
    for member in union_type_def.union_member_types() {
        let member_string = member.to_string();
        let caps = REGEX.global_match_words.find_iter(&member_string);

        for cap in caps {
            if cap.is_ok() {
                member_types.push(cap.unwrap().as_str().to_string());
            }
        }
    }


    let union_type: UnionType = UnionType {
        name: name,
        description: description,
        directives: directives,
        member_types: either!(member_types.len() != 0 => Some(member_types); None),
    };

    return union_type;
}

pub fn parse_union_type_extension(union_type_extension: UnionTypeExtension, union_types: &mut Vec<UnionType>, config: &Config) {
    let mut extends = union_type_extension.name().unwrap().to_string();
    extends = REGEX.match_whitespace.replace_all(&extends, "").to_string();

    // If no match is found, then we simply do nothing this this extension. We won't return an error
    for union_type in union_types {
        if extends == union_type.name {
            match parse_directive_instance(union_type_extension.directives(), config) {
                Some(directives) => {
                    for directive in directives {
                        if let Some(ref mut d) = union_type.directives {
                            d.push(directive);
                        }
                        else {
                            union_type.directives = Some(vec![directive]);
                        }
                    }
                },
                None => {}
            }

            let mut member_types: Vec<String> = vec![];

            for member in union_type_extension.union_member_types() {
                let member_string = member.to_string();
                let caps = REGEX.global_match_words.find_iter(&member_string);

                for cap in caps {
                    if cap.is_ok() {
                        member_types.push(cap.unwrap().as_str().to_string());
                    }
                }
            }

            for member in union_type.member_types.as_ref().unwrap() {
                member_types.push(member.clone());
            }

            union_type.member_types = Some(member_types);
        }
    }
}