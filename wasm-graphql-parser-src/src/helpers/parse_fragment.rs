use apollo_parser::ast::{FragmentDefinition};
use crate::prelude::{Fragment, DirectiveData, Selection, REGEX, parse_directive_instance, parse_selection_set, Config};

pub fn parse_fragment_def(fragment_def: FragmentDefinition, config: &Config) -> Fragment {
    let mut name = REGEX.match_whitespace.replace_all(&fragment_def.fragment_name().unwrap().to_string(), "").to_string();
    name = REGEX.match_whitespace.replace_all(&name, "").to_string();

    let directives: Option<Vec<DirectiveData>> = parse_directive_instance(fragment_def.directives(), config);
    
    let mut selection_sets: Vec<Vec<Selection>> = vec![];

    for selection_set in fragment_def.selection_set() {
        // The apollo parser returns a seleciton set as a whol string, so we have to do manual parsing for that
        selection_sets.push(parse_selection_set(selection_set, config));
    }

    let mut on = REGEX.match_on_word.replace_all(&fragment_def.type_condition().unwrap().to_string(), "").to_string();
    on = REGEX.match_whitespace.replace_all(&on, "").to_string();

    let fragment: Fragment = Fragment {
        name: name,
        directives: directives,
        selection_sets: selection_sets,
        on: on,
    };

    return fragment;
}