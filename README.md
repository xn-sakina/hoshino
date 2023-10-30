# hoshino

Fast string seaching powered by Rust. ( built upon [aho-corasick](https://github.com/BurntSushi/aho-corasick) )

### Install

```bash
  pnpm i -D hoshino
```

### Usage

#### `findAllMatchSync` / `findAllMatch`

```ts
import { findAllMatchSync } from 'hoshino'

const patterns = ['apple', 'maple', 'Snapple']
const matches = findAllMatchSync({
  patterns,
  haystack: 'Nobody likes maple in their apple flavored Snapple.',
  //                      ^ 0 ^          ^ 1 ^          ^  2  ^
})

// pattern
assert(patterns[matches[0].pattern], 'maple')
assert(patterns[matches[1].pattern], 'apple')
assert(patterns[matches[2].pattern], 'Snapple')

// index
const { start, end } = matches[0]
const uint8Arr = new TextEncoder().encode(text)
const matchedUnit8Arr = uint8Arr.slice(start, end)
const matchedText = new TextDecoder().decode(matchedUnit8Arr)
assert(matchedText, 'maple')
```

#### `findLeftFirstMatchSync` / `findLeftFirstMatch`

```ts
import { findLeftFirstMatchSync } from 'hoshino'

const patterns = ['apple', 'maple', 'Snapple']
const { matched, pattern } = findLeftFirstMatchSync({
  patterns,
  haystack: 'Nobody likes maple in their apple flavored Snapple.',
  //                      ^^^^^ finding the leftmost first match
})

if (matched) {
  assert(patterns[pattern], 'maple')
}
```

#### `findLeftFirstLongestMatchSync` / `findLeftFirstLongestMatch`

```ts
import { findLeftFirstLongestMatch } from 'hoshino'

const patterns = ['map', 'maple', 'Snapple']
const { matched, pattern } = await findLeftFirstLongestMatch({
  patterns,
  haystack: 'Nobody likes maple in their apple flavored Snapple.',
  //                      ^^^^^ finding the leftmost-longest first match
})

if (matched) {
  assert(patterns[pattern], 'maple')
}
```

### License

MIT
