import fs from 'node:fs'
import { join } from 'node:path'
// If using --target nodejs as build target, straight up importing magically works
import { FieldData, parse_schema_for_typescript, SchemaData } from 'simple-wasm-graphql-parser'

// Requires --target nodejs
const test_parser = async () => {
  try {
    const schema_data: Buffer = fs.readFileSync(join(__dirname, "./schema.graphql"))
    const raw_schema = schema_data.toString()

    const data = parse_schema_for_typescript(raw_schema) as SchemaData

    if (data.directives && 
        data.enum_types && 
        data.input_object_types && 
        data.interface_types && 
        data.object_types && 
        data.scalar_types &&
        data.union_types) console.log("Simple WASM GraphQL Parser is working")
    else {
      console.error("Simple WASM GraphQL Parser is NOT working")
      process.exit(1)
    }

  }
  catch(e) {
    console.log(e)
  }
}

// Idea of this test is to replicate how a nodejs project would call the parser
// This is more for internal testing of the parser
// Unit tests and such should not be handled in this backend-test package
const main = async () => {
  await test_parser()
}

main()
.then()
.catch(e => {
  console.log(e)
})