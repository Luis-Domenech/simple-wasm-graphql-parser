import fs from 'node:fs'

const rm_dir = async () => {
  try {
    fs.rmSync('./pkg', {recursive: true, force: true})
  }
  catch(e) {
    console.log(e)
  }
}

rm_dir()
.then()
.catch(e => console.log(e))
