import fs from 'node:fs'
import path from 'node:path'

const gen_npmrc = async () => {
  if (!process.env.NPM_TOKEN) {
    console.error("NPM_TOKEN is not set in environment")
    process.exit(1)
  }
  if (!process.env.NPM_USER) {
    console.error("NPM_USER is not set in environment")
    process.exit(1)
  }
  // Create fodler if it doesn't exist
  fs.mkdirSync(path.join(__dirname, "../packages/wasm-graphql-parser"), { recursive: true })

  let file_content = [
    `registry=https://registry.npmjs.org/`,
    ``,
    `@${process.env.NPM_USER}:registry=https://registry.npmjs.org/`,
    `//registry.npmjs.org/:_authToken=${process.env.NPM_TOKEN}`,
    `always-auth=true`
  ].join("\n")

  try {
    fs.writeFileSync(path.join(__dirname, "../packages/wasm-graphql-parser/.npmrc"), file_content)
  }
  catch(e) {
    console.error(e)
    process.exit(1)
  }
}

gen_npmrc()
.then()
.catch(e => console.log(e))
