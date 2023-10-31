import { findAllMatch } from 'hoshino'

const run = async () => {
  const task = async () => {
    const haystack = '中文测试cc是😅é是我是是'
    const patterns = ['我是', '你说搜索', 'é', '😅', 'C']
    const matches = await findAllMatch(
      {
        patterns,
        haystack,
      },
      {
        caseInsensitive: true,
      },
    )
    matches.forEach((mat) => {
      console.log(
        'mat: ',
        mat,
        patterns[mat.pattern!],
        haystack.slice(mat.start!, mat.end!),
      )
    })
  }
  task()
}

run()
