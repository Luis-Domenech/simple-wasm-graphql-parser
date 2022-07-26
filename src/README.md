![integration logo](https://raw.githubusercontent.com/Luis-Domenech/wasm-graphql-parser/main/assets/integration.png)

# wasm-graphql-parser

A very basic GraphQL parser that uses [Apollo's parser](https://crates.io/crates/apollo-parser) as a start and some custom logic to extract all data from a schema. The package in npm was built and published using `wasm-pack`.

## Installation

To install, you can use:

```bash
yarn add --dev wasm-graphql-parser
```
or
```bash
npm install -D wasm-graphql-parser
```

## Description

For now, the package contains a web assembly build that was built using `--target nodejs` as the target for `wasm-pack build`. This means that frontend codebases can't import the package. This is expected behaviour. Future releases might have some way of working with frontend codebases.

Lastly, the package exports a function, which is how the web assembly parser is called, and all the typescript types that are returned from the function. 

## How to use
The main function that is exported to the user is `parse_schema_for_typescript`. It receives a schema in string form and returns a any object. This any object can be typecast to SchemaData, which is what is actually being returned by the functions. All in all, an example of usage could be:

```typescript
import { parse_schema_for_typescript, SchemaData } from 'wasm-graphql-parser'

const schema_data: Buffer = fs.readFileSync(join(__dirname, "./schema.graphql"))
const raw_schema = schema_data.toString()

const data = parse_schema_for_typescript(raw_schema) as SchemaData
```