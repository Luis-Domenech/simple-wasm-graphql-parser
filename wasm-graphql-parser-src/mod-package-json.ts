import fs from 'node:fs'

const mod_package_json = async () => {
  if (!process.env.NODE_AUTH_TOKEN && !process.env.NPM_TOKEN) {
    console.error("NODE_AUTH_TOKEN and NPM_TOKEN are neither set in environment. Set one of them before deploying to prod.")
    process.exit(1)
  }
  // if (!process.env.NPM_USER) {
  //   console.error("NPM_USERis not set in environment")
  //   process.exit(1)
  // }

  let file_content = fs.readFileSync("./pkg/package.json", 'utf-8')

  let content = [
    `,`,
    `  "publishConfig": {`,
    // `    "registry": "https://registry.npmjs.org/:_authToken=$${process.env.NODE_AUTH_TOKEN ? 'process.env.NODE_AUTH_TOKEN' : 'process.env.NPM_TOKEN'}"`,
    // `    "${process.env.NPM_USER}:registry": "https://registry.npmjs.org/"`,
    `    "registry": "https://registry.npmjs.org/"`,
    `  }`,
    `}`
  ].join("\n")

  file_content = file_content.replace(/([\s]*[}]+[\s]*$)/gm, content)

  try {
    fs.writeFileSync("./pkg/package.json", file_content)
  }
  catch(e) {
    console.error(e)
    process.exit(1)
  }
}

mod_package_json()
.then()
.catch(e => console.log(e))
