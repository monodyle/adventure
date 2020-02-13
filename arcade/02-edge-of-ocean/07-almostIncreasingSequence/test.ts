import { almostIncreasingSequence } from './index'

let testCase = [
  {s: [1, 3, 2, 1], result: false},
  {s: [1, 3, 2], result: true},
  {s: [1, 2, 1, 2], result: false},
  {s: [3, 6, 5, 8, 10, 20, 15], result: false},
  {s: [1, 1, 2, 3, 4, 4], result: false},
  {s: [1, 4, 10, 4, 2], result: false},
  {s: [10, 1, 2, 3, 4, 5], result: true},
  {s: [1, 1, 1, 2, 3], result: false},
  {s: [0, -2, 5, 6], result: true},
  {s: [1, 2, 3, 4, 5, 3, 5, 6], result: false},
  {s: [40, 50, 60, 10, 20, 30], result: false},
  {s: [1, 1], result: true},
  {s: [1, 2, 5, 3, 5], result: true},
  {s: [1, 2, 5, 5, 5], result: false},
  {s: [10, 1, 2, 3, 4, 5, 6, 1], result: false},
  {s: [1, 2, 3, 4, 3, 6], result: true},
  {s: [1, 2, 3, 4, 99, 5, 6], result: true},
  {s: [123, -17, -5, 1, 2, 3, 12, 43, 45], result: true},
  {s:[3, 5, 67, 98, 3], result: true},
]

testCase.forEach(i => {
  let result = almostIncreasingSequence(i.s)
  console.log(`${i.s} => ${result} [${result == i.result ? '✔ Passed' : '❌ Fail'}]`)
})