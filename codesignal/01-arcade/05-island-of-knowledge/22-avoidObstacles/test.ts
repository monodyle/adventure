import { avoidObstacles } from './index'

let testCase = [
  {input: [5, 3, 6, 7, 9], result: 4},
  {input: [2, 3], result: 4},
  {input: [1, 4, 10, 6, 2], result: 7},
  {input: [1000, 999], result: 6},
  {input: [19, 32, 11, 23], result: 3},
  {input: [5, 8, 9, 13, 14], result: 6},
]

testCase.forEach(i => {
  let result = avoidObstacles(i.input)
  let passed = result === i.result
  let debug = !passed ? ` - Want: ${i.result} | Given: ${result}` : ''
  console.log(`Input: ${i.input} => ${result} [${passed ? '✔ Passed' : '❌ Fail'}${debug}]`)
})