import { spawnSync } from 'node:child_process'
import { gen_otp } from './gen-otp'

const npm_login = async () => {
  if (!process.env.NPM_TOKEN) {
    console.error("NPM_TOKEN is not set in environment")
    process.exit(1)
  }
  if (!process.env.NPM_USER) {
    console.error("NPM_USER is not set in environment")
    process.exit(1)
  }
  if (!process.env.NPM_EMAIL) {
    console.error("NPM_EMAIL is not set in environment")
    process.exit(1)
  }
  if (!process.env.NPM_PASS) {
    console.error("NPM_USER is not set in environment")
    process.exit(1)
  }

  const otp = await gen_otp()
  console.log(otp)
  console.log(process.env.NPM_2FA_SECRET)
  try {
    // spawnSync(`npm config `, [], { shell: true, stdio: 'inherit' })
    spawnSync(`npm-cli-login -r https://registry.npmjs.org/:_authToken=${process.env.NPM_TOKEN} --otp=${otp}`, [], { shell: true, stdio: 'inherit' })
  }
  catch(e) {
    console.error(e)
    process.exit(1)
  }
}

npm_login()
.then()
.catch(e => console.log(e))
