use std::{env, fs, path};
use apollo_parser::{Parser, ast};

use crate::prelude::{ 
    REGEX,
    ObjectType, 
    ScalarType, 
    EnumType, 
    InterfaceType, 
    OperationType, 
    Directive, 
    UnionType, 
    Fragment, 
    Schema, 
    SchemaData,
    parse_operation_def,
    parse_object_type_def, 
    parse_object_type_extension,
    parse_input_object_type_def, 
    parse_input_object_type_extension,
    parse_scalar_type_def, 
    parse_scalar_type_extension,
    parse_enum_type_def, 
    parse_enum_type_extension,
    parse_interface_type_def, 
    parse_interface_type_extension,
    parse_union_type_def, 
    parse_union_type_extension,
    parse_directive_def,
    parse_fragment_def,
    parse_schema_def, 
    parse_schema_extension,
    either, 
    Logger,
    process_fields, 
    par_process_fields, 
    Config
};

// This takes a schema as string and runs it through Apollo's Rust parser
// It works well enough, but it doesn't parse many things
// So we take care of thr rest of the parsing and saves all the data in logical structs

// Note that validation of schema is done by apollo's parser
// However, there parser only looks at syntactical errors
// Logical errors like using directives that haven't been declared
// and types that implement interfaces but don't implement there fields
// are not hanlded by Apollo's parser, and this parser either
pub fn parse_schema_from_string(raw_schema: &str, config: &Config) -> SchemaData { 
    // Remove comments, as in, everything with #
    // Descriptions are not removed, as in anything with "
    let mut schema = REGEX.global_match_comments.replace_all(&raw_schema, "").to_string();

    // Convert all \r\n to \n
    schema = REGEX.global_match_carriage_return_and_new_line.replace_all(&schema, "\n").to_string();

    parse_entire_schema(&schema, config)
}


pub fn parse_schema_from_file(schema_file_path: &str, config: &Config) -> SchemaData { 
    // This is from root of project
    let dir = env::current_dir().expect("Error getting current directory path");
    let __dirname = dir.to_str().expect("Error getting current directory path");
    let schema_path_buf = path::Path::new(__dirname).join(schema_file_path);
    let absolute_schema_file_path = schema_path_buf.to_str().expect("Error joining paths with path join");
    let raw_uncleaned_schema = fs::read_to_string(absolute_schema_file_path).expect("Error reading schema");

    // Remove comments, as in, everything with #
    // Descriptions are not removed, as in anything with "
    let mut raw_schema = REGEX.global_match_comments.replace_all(&raw_uncleaned_schema, "").to_string();

    // Convert all \r\n to \n
    raw_schema = REGEX.global_match_carriage_return_and_new_line.replace_all(&raw_schema, "\n").to_string();

    parse_entire_schema(&raw_schema, config)
}



fn parse_entire_schema(raw_schema: &str, config: &Config) -> SchemaData {
    let mut operation_types: Vec<OperationType> = vec![];
    let mut object_types: Vec<ObjectType> = vec![];
    let mut input_object_types: Vec<ObjectType> = vec![];
    let mut scalar_types: Vec<ScalarType> = vec![];
    let mut enum_types: Vec<EnumType> = vec![];
    let mut interface_types: Vec<InterfaceType> = vec![];
    let mut union_types: Vec<UnionType> = vec![];
    let mut directives: Vec<Directive> = vec![];
    let mut fragments: Vec<Fragment> = vec![];
    let mut schemas: Vec<Schema> = vec![];

    let parser = Parser::new(&raw_schema);
    // The apollo parser is is error resilient, so it will always return an ast along with errors encountered
    let ast = parser.parse();

    // ast.errors() returns an iterator with the errors encountered during lexing and parsing
    if ast.errors().len() > 0 {
        for error in ast.errors() {
            println!("{:#?}", error);
            Logger::error(error.message(), None, config);
        }
    }

    // ast.document() gets the Document, or root node, of the tree that you can start iterating on.
    let doc = ast.document();

    // Now we iterate the ast doc
    for def in doc.definitions() {
        match def {
            // Operations
            // TODO: Fix this since operations only appear here if specified in a schema
            // We must make sure to handle cases where both type and operation is provided
            // For now, all operation types are handled by the parse_object_type_def
            ast::Definition::OperationDefinition(operation_def) => {
                operation_types.push(parse_operation_def(operation_def, config));
            }

            // Object Types
            ast::Definition::ObjectTypeDefinition(object_type_def) => { 
                object_types.push(parse_object_type_def(object_type_def, config));
            }

            // Scalars
            ast::Definition::ScalarTypeDefinition(scalar_type_def) => {
                scalar_types.push(parse_scalar_type_def(scalar_type_def, config));
            }

            // Enums
            ast::Definition::EnumTypeDefinition(enum_type_def) => {
                enum_types.push(parse_enum_type_def(enum_type_def, config));
            }

            // Interfaces
            ast::Definition::InterfaceTypeDefinition(interface_type_def) => {
                interface_types.push(parse_interface_type_def(interface_type_def, config));
            }

            // Input Object Types
            ast::Definition::InputObjectTypeDefinition(input_object_type_def) => {
                input_object_types.push(parse_input_object_type_def(input_object_type_def, config));
            }

            // Unions
            ast::Definition::UnionTypeDefinition(union_type_def) => {
                union_types.push(parse_union_type_def(union_type_def, config));
            }

            // Directives
            ast::Definition::DirectiveDefinition(directive_def) => {
                directives.push(parse_directive_def(directive_def, config));
            }

            // Fragments
            ast::Definition::FragmentDefinition(fragment_def) => {
                fragments.push(parse_fragment_def(fragment_def, config));
            }

            // Schema Definitions
            ast::Definition::SchemaDefinition(schema_def) => {
                schemas.push(parse_schema_def(schema_def, config));
            }

            // Type Extensions
            ast::Definition::ObjectTypeExtension(object_type_extension) => {
                parse_object_type_extension(object_type_extension, &mut object_types, config);
            }

            ast::Definition::ScalarTypeExtension(scalar_type_extension) => {
                parse_scalar_type_extension(scalar_type_extension, &mut scalar_types, config);
            }

            ast::Definition::EnumTypeExtension(enum_type_extension) => {
                parse_enum_type_extension(enum_type_extension, &mut enum_types, config);
            }

            ast::Definition::InterfaceTypeExtension(interface_type_extension) => {
                parse_interface_type_extension(interface_type_extension, &mut interface_types, config);
            }

            ast::Definition::InputObjectTypeExtension(input_object_type_extension) => {
                parse_input_object_type_extension(input_object_type_extension, &mut input_object_types, config);
            }

            ast::Definition::UnionTypeExtension(union_type_extension) => {
                parse_union_type_extension(union_type_extension, &mut union_types, config);
            }

            ast::Definition::SchemaExtension(schema_extension) => {
                parse_schema_extension(schema_extension, &mut schemas, config);
            }
        }
    }

    let mut schema_data: SchemaData = SchemaData {
        operation_types: either!(operation_types.len() > 0 => Some(operation_types); None),
        object_types: either!(object_types.len() > 0 => Some(object_types); None),
        input_object_types: either!(input_object_types.len() > 0 => Some(input_object_types); None),
        scalar_types: either!(scalar_types.len() > 0 => Some(scalar_types); None),
        enum_types: either!(enum_types.len() > 0 => Some(enum_types); None),
        interface_types: either!(interface_types.len() > 0 => Some(interface_types); None),
        union_types: either!(union_types.len() > 0 => Some(union_types); None),
        directives: either!(directives.len() > 0 => Some(directives); None),
        fragments: either!(fragments.len() > 0 => Some(fragments); None),
        schemas: either!(schemas.len() > 0 => Some(schemas); None)
    };


    // Now we go through every field and add data for is_enum, is_scalar
    if config.parallel { par_process_fields(&mut schema_data, config); } else { process_fields(&mut schema_data, config); }

    return schema_data
}