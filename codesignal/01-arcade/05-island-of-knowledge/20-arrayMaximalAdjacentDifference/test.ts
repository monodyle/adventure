import { arrayMaximalAdjacentDifference } from './index'

let testCase = [
  {input: [2, 4, 1, 0], result: 3},
  {input: [1, 1, 1, 1], result: 0},
  {input: [-1, 4, 10, 3, -2], result: 7},
  {input: [10, 11, 13], result: 2},
  {input: [-2, -2, -2, -2, -2], result: 0},
  {input: [-1, 1, -3, -4], result: 4},
]

testCase.forEach(i => {
  let result = arrayMaximalAdjacentDifference(i.input)
  let passed = result === i.result
  let debug = !passed ? ` - Want: ${i.result} | Given: ${result}` : ''
  console.log(`Input: ${i.input} => ${result} [${passed ? '✔ Passed' : '❌ Fail'}${debug}]`)
})