import { findAllMatch, findAllMatchSync } from 'hoshino'

const run = async () => {
  const task = async () => {
    const haystack = '中文测试cc是😅é是我是是'
    const matches = await findAllMatch(
      {
        patterns: ['我是', '你说搜索', 'é', '😅', 'C'],
        haystack,
      },
      {
        caseInsensitive: true,
      },
    )
    matches.forEach((mat) => {
      console.log('mat: ', mat)
    })
  }
  task()
}

run()
