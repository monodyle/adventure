import { isLucky } from './index'

let testCase = [
  {input: 1230, result: true},
  {input: 239017, result: false},
  {input: 134008, result: true},
  {input: 10, result: false},
  {input: 11, result: true},
  {input: 1010, result: true},
  {input: 261534, result: false},
  {input: 100000, result: false},
  {input: 999999, result: true},
  {input: 123321, result: true},
]
testCase.forEach(i => {
  let result = isLucky(i.input)
  let passed = result == i.result
  let debug = !passed ? ` - Want: ${i.result} | Given: ${result}` : ''
  console.log(`${i.input} [${passed ? '✔ Passed' : '❌ Fail'}${debug}]`)
})