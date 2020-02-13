import { makeArrayConsecutive2 } from './index'

let testCase = [
  {s: [6, 2, 3, 8], result: 3},
  {s: [0, 3], result: 2},
  {s: [5, 4, 6], result: 0},
  {s: [6, 3], result: 2},
  {s: [1], result: 0}
]

testCase.forEach(i => {
  let result = makeArrayConsecutive2(i.s)
  console.log(`${i.s} => ${result} [${result == i.result ? '✔ Passed' : '❌ Fail'}]`)
})