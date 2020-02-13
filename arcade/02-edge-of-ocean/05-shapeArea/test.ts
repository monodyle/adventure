import { shapeArea } from './index'

let testCase = [
  {n: 2, result: 5},
  {n: 3, result: 13},
  {n: 1, result: 1},
  {n: 5, result: 41},
  {n: 7000, result: 97986001},
  {n: 8000, result: 127984001},
  {n: 9999, result: 199940005},
  {n: 9998, result: 199900013},
  {n: 8999, result: 161946005},
  {n: 100, result: 19801},
]

testCase.forEach(i => {
  let result = shapeArea(i.n)
  console.log(`${i.n} => ${result} [${result == i.result ? '✔ Passed' : '❌ Fail'}]`)
})