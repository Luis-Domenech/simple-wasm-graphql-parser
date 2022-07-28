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
`    "registry": "https://registry.npmjs.org/"`,
`  }`,
`  "homepage": "https://github.com/Luis-Domenech/wasm-graphql-parser",`,
`  "author": "luis-domenech <luisfabiandomenech@gmail.com>",`,
`  "keywords": [`,
`    "rust",`,
`    "webassembly",`,
`    "wasm",`,
`    "graphql",`,
`    "parser",`,
`    "schema",`,
`    "typescript"`,
`  ],`,
`  "release": {`,
`    "branches": [`,
`      "main"`,
`    ],`,
`    "plugins": [`,
`      "@semantic-release/commit-analyzer",`,
`      "@semantic-release/release-notes-generator",`,
`      [`,
`        "@semantic-release/changelog",`,
`        {`,
`          "changelogFile": "../CHANGELOG.md"`,
`        }`,
`      ],`,
`      "@semantic-release/npm",`,
`      "@semantic-release/github",`,
`      [`,
`        "@semantic-release/git",`,
`        {`,
`          "assets": [`,
`            "../CHANGELOG.md",`,
`            "../package.json"`,
`          ],`,
`          "message": "chore(release): set \`package.json\` to \${nextRelease.versio} [skip ci]\n\n\${nextRelease.notes}"`,
`        }`,
`      ]`,
`    ]`,
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
