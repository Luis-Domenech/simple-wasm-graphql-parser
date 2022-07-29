import fs from 'node:fs'

const mod_pkg_json = async () => {
  let file_content = fs.readFileSync("./pkg/package.json", 'utf-8')

  let content = [
`,`,
`  "publishConfig": {`,
`    "registry": "https://registry.npmjs.org/"`,
`  },`,
`  "homepage": "https://github.com/Luis-Domenech/simple-wasm-graphql-parser",`,
// `  "author": "Luis F. Domenech Ortiz <luisfabiandomenech@gmail.com> (https://luisfdomenech.com)",`,
`  "keywords": [`,
`    "rust",`,
`    "webassembly",`,
`    "wasm",`,
`    "graphql",`,
`    "parser",`,
`    "schema",`,
`    "typescript"`,
`  ]`,
// `  ],`,
// `  "release": {`,
// `    "branches": [`,
// `      "main"`,
// `    ],`,
// `    "plugins": [`,
// `      "@semantic-release/commit-analyzer",`,
// `      "@semantic-release/release-notes-generator",`,
// `      [`,
// `        "@semantic-release/changelog",`,
// `        {`,
// `          "changelogFile": "../CHANGELOG.md"`,
// `        }`,
// `      ],`,
// `      "@semantic-release/npm",`,
// `      "@semantic-release/github",`,
// `      [`,
// `        "@semantic-release/git",`,
// `        {`,
// `          "assets": [`,
// `            "../CHANGELOG.md",`,
// `            "../package.json"`,
// `          ],`,
// `          "message": "chore(release): set \`package.json\` to \${nextRelease.versio} [skip ci]\\n\\n\${nextRelease.notes}"`,
// `        }`,
// `      ]`,
// `    ]`,
// `  }`,
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

mod_pkg_json()
.then()
.catch(e => console.log(e))
