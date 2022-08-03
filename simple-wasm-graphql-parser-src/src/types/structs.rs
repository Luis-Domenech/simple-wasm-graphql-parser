use fancy_regex::Regex;
use apollo_parser::ast::{FieldDefinition, InputValueDefinition, VariableDefinition};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub parallel: bool,
    pub run_in_wasm: bool
}


// The Debug is not from serde. It is so that we can print structs to console
#[derive(Serialize, Deserialize, Debug)]
pub struct FieldData {
    pub name: String,
    pub field_type: String,
    pub field_complete_type: String,
    pub directives: Option<Vec<DirectiveData>>,
    pub description: Option<String>,
    pub arguments: Option<Vec<FieldArgumentData>>,
    pub default_value: Option<String>,
    pub is_nullable: bool,
    pub is_array: bool,
    pub is_enum: bool,
    pub is_scalar: bool,
    pub is_union: bool,
}


// #[wasm_bindgen]
// pub struct Bazzzz {
//     field: String,
//     another: bool
// }

// #[wasm_bindgen]
// impl Bazzzz {
//     #[wasm_bindgen(constructor)]
//     pub fn new(field: String, another: bool) -> Bazzzz {
//         Bazzzz { field, another }
//     }

//     #[wasm_bindgen(getter)]
//     pub fn field(&self) -> String { self.field.to_string() }
//     #[wasm_bindgen(setter)]
//     pub fn set_field(&mut self, field: String) { self.field = field; }

//     #[wasm_bindgen(getter)]
//     pub fn another(&self) -> bool { self.another }
//     #[wasm_bindgen(setter)]
//     pub fn set_another(&mut self, another: bool) { self.another = another; }
// }


#[derive(Serialize, Deserialize, Debug)]
pub struct FieldArgumentData {
    pub argument_index: usize,
    pub name: String,
    pub argument_type: String,
    pub argument_complete_type: String,
    pub directives: Option<Vec<DirectiveData>>,
    pub description: Option<String>,
    pub default_value: Option<String>,
    pub is_nullable: bool,
    pub is_array: bool,
    pub is_enum: bool,
    pub is_scalar: bool,
    pub is_union: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectType {
    pub name: String,
    pub fields: Vec<FieldData>,
    pub object_type: String,
    pub directives: Option<Vec<DirectiveData>>,
    pub description: Option<String>,
    pub implements: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationType {
    pub name: String,
    pub fields: Vec<FieldData>,
    pub operation_type: String,
    pub directives: Option<Vec<DirectiveData>>,
    pub selection_sets: Vec<Vec<Selection>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Selection {
    pub name: String,
    pub selections: Option<Vec<Selection>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Directive {
    pub name: String,
    pub repeatable: bool,
    pub description: Option<String>,
    pub arguments: Option<Vec<FieldArgumentData>>,
    pub locations: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnionType {
    pub name: String,
    pub description: Option<String>,
    pub directives: Option<Vec<DirectiveData>>,
    pub member_types: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fragment {
    pub name: String,
    pub directives: Option<Vec<DirectiveData>>,
    pub selection_sets: Vec<Vec<Selection>>,
    pub on: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    pub directives: Option<Vec<DirectiveData>>,
    pub description: Option<String>,
    pub root_operations: Vec<SchemaOperationType>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaOperationType {
    pub name: String,
    pub operation_type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScalarType {
    pub name: String,
    pub directives: Option<Vec<DirectiveData>>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnumType {
    pub name: String,
    pub values: Vec<EnumValueData>,
    pub directives: Option<Vec<DirectiveData>>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InterfaceType {
    pub name: String,
    pub fields: Vec<FieldData>,
    pub directives: Option<Vec<DirectiveData>>,
    pub description: Option<String>,
    pub implements: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectiveData {
    pub name: String,
    pub values: Option<Vec<DirectiveArgumentValues>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectiveArgumentValues {
    pub parameter: String,
    pub value: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaData {
    pub operation_types: Option<Vec<OperationType>>,
    pub object_types: Option<Vec<ObjectType>>,
    pub input_object_types: Option<Vec<ObjectType>>,
    pub scalar_types: Option<Vec<ScalarType>>,
    pub enum_types: Option<Vec<EnumType>>,
    pub interface_types: Option<Vec<InterfaceType>>,
    pub union_types: Option<Vec<UnionType>>,
    pub directives: Option<Vec<Directive>>,
    pub fragments: Option<Vec<Fragment>>,
    pub schemas: Option<Vec<Schema>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnumValueData {
    pub value: String,
    pub description: Option<String>,
    pub directives: Option<Vec<DirectiveData>>
}

// Used for parse_fields as a union of types for a parameter of the function parse_fields
pub enum FieldDefs {
    FieldDef(FieldDefinition),
    InputValueDef(InputValueDefinition),
    VariableDef(VariableDefinition),
}

// For parsing stuff
pub struct RegexPattern {
    pub global_match_words: Regex,
    pub global_match_comments: Regex,
    pub global_match_description_with_quotes: Regex,
    pub global_match_carriage_return_and_new_line: Regex,
    pub match_comments_before_type: Regex,
    pub match_complete_type: Regex,
    pub match_exclamations: Regex,
    pub match_closing_brackets: Regex,
    pub match_brackets_and_exclamation: Regex,
    pub match_description_beginning_end_quotes: Regex,
    pub match_comment_at_line_end: Regex,
    pub match_all_directives_with_parenthesis: Regex,
    pub match_directive_name: Regex,
    pub match_whitespace: Regex,
    pub match_linebreak: Regex,
    pub match_on_word: Regex,
    pub match_first_word: Regex,
    pub match_word: Regex,
    pub match_single_quote_description: Regex,
    pub match_all_parenthesis: Regex,
    pub match_field_name: Regex,
    pub match_directive_argument_value: Regex,
}





