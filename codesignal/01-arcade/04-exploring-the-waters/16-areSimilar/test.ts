import { areSimilar } from './index'
var array = require('lodash/array');

let testCase = [
  {a: [1, 2, 3], b: [1, 2, 3], result: true},
  {a: [1, 2, 3], b: [1, 2, 3], result: true},
  {a: [1, 2, 2], b: [2, 1, 1], result: false},
  {a: [1, 1, 4], b: [1, 2, 3], result: false},
  {a: [1, 2, 3], b: [1, 10, 2], result: false},
  {a: [2, 3, 1], b: [1, 3, 2], result: true},
  {a: [2, 3, 9], b: [10, 3, 2], result: false},
  {a: [4, 6, 3], b: [3, 4, 6], result: false},
  {a: [832, 998, 148, 570, 533, 561, 894, 147, 455, 279], b: [832, 998, 148, 570, 533, 561, 455, 147, 894, 279], result: true},
  {a: [832, 998, 148, 570, 533, 561, 894, 147, 455, 279], b: [832, 570, 148, 998, 533, 561, 455, 147, 894, 279], result: false},
]

testCase.forEach(i => {
  let result = areSimilar(i.a, i.b)
  let passed = result === i.result
  let debug = !passed ? ` - Want: ${i.result} | Given: ${result}` : ''
  console.log(`Input: ${i.a}, ${i.b} [${passed ? '✔ Passed' : '❌ Fail'}${debug}]`)
})