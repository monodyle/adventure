import { sortByHeight } from './index'

let testCase = [
  {input: [-1, 150, 190, 170, -1, -1, 160, 180], result: [-1, 150, 160, 170, -1, -1, 180, 190]},
  {input: [-1, -1, -1, -1, -1], result: [-1, -1, -1, -1, -1]},
  {input: [-1], result: [-1]},
  {input: [4, 2, 9, 11, 2, 16], result: [2, 2, 4, 9, 11, 16]},
  {input: [2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 1], result: [1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 2]},
  {input: [23, 54, -1, 43, 1, -1, -1, 77, -1, -1, -1, 3], result: [1, 3, -1, 23, 43, -1, -1, 54, -1, -1, -1, 77]},
]

testCase.forEach(i => {
  let result = sortByHeight(i.input)
  let passed = result.every(e => i.result.includes(e)) && i.result.every((e) => result.includes(e))
  let debug = !passed ? ` - Want: ${i.result} | Given: ${result}` : ''
  console.log(`Input: ${i.input} [${passed ? '✔ Passed' : '❌ Fail'}${debug}]`)
})