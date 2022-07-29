![integration logo](https://raw.githubusercontent.com/Luis-Domenech/wasm-graphql-parser/main/assets/imgs/integration.png)

# wasm-graphql-parser

A very simple GraphQL parser that uses [Apollo's Rust parser](https://crates.io/crates/apollo-parser) as a start and does some extra stuff to extract all data from a graphql schema. The NPM package was built using [`wasm-pack`](https://github.com/rustwasm/wasm-pack).

## Installation

To install, you can run:

```bash
yarn add --dev wasm-graphql-parser
```
or
```bash
npm install -D wasm-graphql-parser
```

## Description

The idea of this package was to create a GraphQL parser. I basically just took the output of Apollo's parser and parsed that output myself. Apollo's parser works phenomenally, but it does't fully parse all data in a schema. For example, enum values can have directives and descriptions attached to each enum value, but their parser only provides enum values directly and all else like descriptions are not given. This parser basically takes into account most issues Apollo's parser has and fixes them in order to return an object with absolutely all data from a provided schema.

For now, the package contains a web assembly build that was built using `--target nodejs` as the target for `wasm-pack build`. This means that frontend codebases can't import the package. This is expected behaviour. Future releases might have some way of working with frontend codebases.

Lastly, the package exports a function, which is how the web assembly parser is called, and all the typescript types that are returned from the function. 

## How to use
This package exports two things. First is the function, `parse_schema_for_typescript`. It receives a schema in string form and returns an object of type `any`. As I understand, the complex type that is returned by that function can't be exported automatically to TyepScript by `wasm-bindgen` and `serde`, so the function returns an `any` object. However, the return object does have a type, which is `SchemaData`, and this is the second export of the package. Basically, you can import all data types that are associated with the return object. This return object can be typecast to `SchemaData`. All in all, some usage example would be:

```typescript
import { parse_schema_for_typescript, SchemaData } from 'wasm-graphql-parser'

const schema_data: Buffer = fs.readFileSync(join(__dirname, "./schema.graphql"))
const raw_schema = schema_data.toString()

const data = parse_schema_for_typescript(raw_schema) as SchemaData

// Prints all object types with some of their info
if (data.object_types) {
    for (const obj of data.object_types) {
        const all_implement = obj.implements ? obj.implements.join(", ") : ""
        const directives = obj.directives ? obj.directives.map(directive => '@' + directive.name + (directive.values ? "(" + directive.values.map(val => `${val.value}: ${val.parameter}`).join(", ") + ")" : '')).join(" ") : ""
        const fields = obj.fields ? obj.fields.map(field => "  " + field.name + ": " + field.field_complete_type).join(",\n") : ""
        
        let str = [
          obj.description,
          `type ${obj.name}${all_implement ? ' implements ' + all_implement : ""}${directives ? " " + directives + " " : " " }{`,
          fields,
          `}`
        ].filter(Boolean).join("\n")

        console.log(`${str}\n`)
    }
}
```

## Notes
This parser has no logical validation. If given a wrongly formatted graphql schema, it will throw an error, but if given sintactically sound schema, it won't throw error based on what's actually inside. For example, you have to decalre a directive before using it. If you start using a directive out of nowhere without first declaring it somewhere in the schema, that should produce an error since it doesn't make sense to use a directive that doesn't exists. Issues like that are not dealth with in Apollo's parser and mine. However, these issues can be easily found with all the data that is produced by the parser, so a schema validation step after parsing a schema can be done, but I don't have the time to do that, so for now, that logical schema validation is left as future work.