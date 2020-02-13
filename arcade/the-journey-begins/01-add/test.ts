import { add } from './index'

let testCase = [
  {a: 1, b: 2, result: 3},
  {a: 0, b: 1000, result: 1000},
  {a: 2, b: -39, result: -37},
  {a: 99, b: 100, result: 199},
  {a: -100, b: 100, result: 0},
  {a: -1000, b: -1000, result: -2000},
]

testCase.forEach(i => {
  let total = add(i.a, i.b)
  console.log(`${i.a} + ${i.b} = ${total} [${total == i.result ? '✔ Passed' : '❌ Fail'}]`)
})