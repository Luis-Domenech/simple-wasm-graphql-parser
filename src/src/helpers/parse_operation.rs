use apollo_parser::ast::{OperationDefinition};
use crate::prelude::{OperationType, DirectiveData, Selection, REGEX, parse_directive_instance, parse_field, FieldDefs, Config};

pub fn parse_operation_def(operation_def: OperationDefinition, config: &Config) -> OperationType {

    let mut name = operation_def.name().unwrap().text().to_string();
    name = REGEX.match_whitespace.replace_all(&name, "").to_string();
    
    let operation_type_type = operation_def.operation_type().unwrap().to_string();

    let directives: Option<Vec<DirectiveData>> = parse_directive_instance(operation_def.directives(), config);
    
    let mut selection_sets: Vec<Vec<Selection>> = vec![];

    for selection_set in operation_def.selection_set() {        
        let mut selections: Vec<Selection> = vec![];

        for selection in selection_set.selections() {
            let sel = Selection {
                name: selection.to_string(),
                selections: None
            };
            selections.push(sel);
        }

        selection_sets.push(selections);
    }

    let mut operation_type = OperationType {
        name: name,
        operation_type: operation_type_type,
        fields: vec![],
        directives: directives,
        selection_sets: selection_sets
    };

    for field_def in operation_def.variable_definitions().unwrap().variable_definitions() {
        operation_type.fields.push(parse_field(FieldDefs::VariableDef(field_def), config));
    }

    return operation_type;
}