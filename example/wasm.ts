import { findAllMatch } from '../target/wasm_publish'

const run = async () => {
  const task = async () => {
    const haystack = '22133'
    const patterns = ['1']
    const matches = await findAllMatch({
      patterns,
      haystack,
    })
    console.log('matches: ', matches)
  }
  task()
}

run()
