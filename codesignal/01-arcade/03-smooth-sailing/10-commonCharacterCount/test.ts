import { commonCharacterCount } from './index'

let testCase = [
  {a: "aabcc", b: "adcaa", result: 3},
  {a: "zzzz", b: "zzzzzzz", result: 4},
  {a: "abca", b: "xyzbac", result: 3},
  {a: "a", b: "b", result: 0},
  {a: "a", b: "aaa", result: 1},
]
testCase.forEach(i => {
  let result = commonCharacterCount(i.a, i.b)
  let passed = result == i.result
  let debug = !passed ? ` - Want: ${i.result} | Given: ${result}` : ''
  console.log(`${i.a} / ${i.b} [${passed ? '✔ Passed' : '❌ Fail'}${debug}]`)
})