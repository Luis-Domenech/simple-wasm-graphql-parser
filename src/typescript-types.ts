
export interface FieldData {
  name: string,
  field_type: string,
  field_complete_type: string,
  directives: DirectiveData[] | null,
  description: string | null,
  arguments: FieldArgumentData[] | null,
  default_value: string | null,
  is_nullable: boolean,
  is_optional: boolean,
  is_array: boolean,
  is_enum: boolean,
  is_scalar: boolean,
  is_union: boolean
}

export interface FieldArgumentData {
  argument_index: number,
  name: string,
  argument_type: string,
  argument_complete_type: string,
  directives: DirectiveData[] | null,
  description: string | null,
  default_value: string | null,
  is_nullable: boolean,
  is_optional: boolean,
  is_array: boolean,
  is_enum: boolean,
  is_scalar: boolean,
  is_union: boolean
}

export interface ObjectType {
  name: string,
  fields: FieldData[],
  object_type: string,
  directives: DirectiveData[] | null,
  description: string | null,
  implements: string[] | null,
}


export interface OperationType {
  name: string,
  fields: FieldData[],
  operation_type: string,
  directives: DirectiveData[] | null,
  selection_sets: Selection[][],
}


export interface Selection {
  name: string,
  selections: Selection[] | null
}


export interface Directive {
  name: string,
  repeatable: boolean,
  description: string | null,
  arguments: FieldArgumentData[] | null,
  locations: string[],
}


export interface UnionType {
  name: string,
  description: string | null,
  directives: DirectiveData[] | null,
  member_types: string[] | null
}


export interface Fragment {
  name: string,
  directives: DirectiveData[] | null,
  selection_sets: Selection[][],
  on: string
}


export interface Schema {
  directives: DirectiveData[] | null,
  description: string | null,
  root_operations: SchemaOperationType[]
}


export interface SchemaOperationType {
  name: string,
  operation_type: string
}


export interface ScalarType {
  name: string,
  directives: DirectiveData[] | null,
  description: string | null,
}


export interface EnumType {
  name: string,
  values: EnumValueData[],
  directives: DirectiveData[] | null,
  description: string | null,
}


export interface InterfaceType {
  name: string,
  fields: FieldData[],
  directives: DirectiveData[] | null,
  description: string | null,
  implements: string[] | null
}


export interface DirectiveData {
  name: string,
  values: DirectiveArgumentValues[] | null
}


export interface DirectiveArgumentValues {
  parameter: string,
  value: string
}


export interface SchemaData {
  operation_types: OperationType[] | null,
  object_types: ObjectType[] | null,
  input_object_types: ObjectType[] | null,
  scalar_types: ScalarType[] | null,
  enum_types: EnumType[] | null,
  interface_types: InterfaceType[] | null,
  union_types: UnionType[] | null,
  directives: Directive[] | null,
  fragments: Fragment[] | null,
  schemas: Schema[] | null
}


export interface EnumValueData {
  value: string,
  description: string | null,
  directives: DirectiveData[] | null
}