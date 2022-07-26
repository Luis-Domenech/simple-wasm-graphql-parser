use apollo_parser::ast::{SchemaDefinition, SchemaExtension};
use crate::prelude::{Schema, DirectiveData, SchemaOperationType, REGEX, parse_directive_instance, Config};

pub fn parse_schema_def(schema_def: SchemaDefinition, config: &Config) -> Schema {
    let description = match schema_def.description() {
        Some(desc) => Some(REGEX.match_description_beginning_end_quotes.replace_all(&desc.to_string(), "").to_string()),
        None => None,
    };

    let directives: Option<Vec<DirectiveData>> = parse_directive_instance(schema_def.directives(), config);
    
    let mut root_operations: Vec<SchemaOperationType> = vec![];

    for root_operation_type in schema_def.root_operation_type_definitions() {
        let root_operation: SchemaOperationType = SchemaOperationType {
            name: root_operation_type.to_string(),
            operation_type: root_operation_type.named_type().unwrap().name().unwrap().to_string(),
        };

        root_operations.push(root_operation);
    }

    // Since we have no names for schemas and we can have multiple schemas in one file, we must create some id for every schema

    let schema: Schema = Schema {
        description: description,
        directives: directives,
        root_operations: root_operations,
    };

    return schema;
}

pub fn parse_schema_extension(_schema_extension: SchemaExtension, _schemas: &mut Vec<Schema>, config: &Config) {
    // I don't think schema extensions are a thing, especially since they have no name attached, and thus, we have no way of knowing which schema is which
    return;
}