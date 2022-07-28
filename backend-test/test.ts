import fs, { cp } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'
import { parse_schema_for_typescript, SchemaData } from 'wasm-graphql-parser'

// Requires --target nodejs
// If using target nodejs, straight up importing works
const loadWasmInNodeJS = async () => {

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
        data.union_types) console.log("WASM GraphQL Parser is working")
    else {
      console.error("WASM GraphQL Parser is NOT working")
      process.exit(1)
    }
  }
  catch(e) {
    console.log(e)
  }
}

const main = async () => {
  await loadWasmInNodeJS()
}


main()
.then()
.catch(e => {
  console.log(e)
})