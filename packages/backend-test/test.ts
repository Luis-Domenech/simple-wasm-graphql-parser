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

    Object.keys(data).map(d => console.log(d))

    // Horrible test here
    if (data.directives.length > 0 && 
        data.enum_types.length > 0 && 
        data.input_object_types.length > 0 && 
        data.interface_types.length > 0 && 
        data.object_types.length > 0 && 
        data.scalar_types.length > 0 &&
        data.union_types.length > 0) console.log("WASM GraphQL Parser is working")
    else console.error("WASM GraphQL Parser is NOT working"); process.exit(1)
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