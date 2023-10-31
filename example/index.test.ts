import {
  findAllMatch,
  findLeftFirstLongestMatch,
  findLeftFirstMatch,
} from 'hoshino'
import { test, expect } from 'vitest'

const getText = (start: number, end: number, text: string) => {
  return text.slice(start, end)
}

test('findAllMatch', async () => {
  const haystack = '中文测试cc是😅é是我是是'
  const matches = await findAllMatch({
    patterns: ['我是', '你说搜索', 'é', '😅', 'C'],
    haystack,
  })
  expect(matches[0].pattern).toEqual(3)
  expect(matches.length).toEqual(3)
  expect(getText(matches[0].start!, matches[0].end!, haystack)).toEqual('😅')
  expect(getText(matches[1].start!, matches[1].end!, haystack)).toEqual('é')
})

test('findLeftFirstLongestMatch', async () => {
  const haystack = '中文测试cc是我是搜索11é是我😅是你说搜索11是'
  const result = await findLeftFirstLongestMatch({
    patterns: ['我是', '我是搜索11', 'é', '😅', 'C'],
    haystack,
  })
  expect(result.matched).toEqual(true)
  expect(result.pattern).toEqual(1)
  expect(getText(result.start!, result.end!, haystack)).toEqual('我是搜索11')
})

test('findLeftFirstMatch', async () => {
  const haystack = '中文测试cc是😅é是我是你说搜索是'
  const result = await findLeftFirstMatch({
    patterns: ['我是', '你说搜索', 'é', '😅', 'C'],
    haystack,
  })
  expect(result.matched).toEqual(true)
  expect(result.pattern).toEqual(3)
  expect(getText(result.start!, result.end!, haystack)).toEqual('😅')
})

test('findLeftFirstMatch - caseInsensitive', async () => {
  const haystack = '中文测试cc是😅é是我是你说搜索是'
  const result = await findLeftFirstMatch(
    {
      patterns: ['我是', '你说搜索', 'é', '😅', 'C'],
      haystack,
    },
    {
      caseInsensitive: true,
    },
  )
  expect(result.matched).toEqual(true)
  expect(result.pattern).toEqual(4)
  expect(getText(result.start!, result.end!, haystack)).toEqual('c')
})
