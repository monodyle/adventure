import { alternatingSums } from './index'
var array = require('lodash/array');

let testCase = [
  {input: [50, 60, 60, 45, 70], result: [180, 105]},
  {input: [100, 50], result: [100, 50]},
  {input: [80], result: [80, 0]},
  {input: [100, 50, 50, 100], result: [150, 150]},
  {input: [100, 51, 50, 100], result: [150, 151]},
]

testCase.forEach(i => {
  let result = alternatingSums(i.input)
  let passed = array.difference(result, i.result).length === 0
  let debug = !passed ? ` - Want: ${i.result} | Given: ${result}` : ''
  console.log(`Input: ${i.input} [${passed ? '✔ Passed' : '❌ Fail'}${debug}]`)
})