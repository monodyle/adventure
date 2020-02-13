import { matrixElementsSum } from './index'

let testCase = [
  { m: [[0, 1, 1, 2], [0, 5, 0, 0], [2, 0, 3, 3]], result: 9 },
  { m: [[1, 1, 1, 0], [0, 5, 0, 1], [2, 1, 3, 10]], result: 9 },
  { m: [[1, 1, 1], [2, 2, 2], [3, 3, 3]], result: 18 },
  { m: [[0]], result: 0 },
  { m: [[1, 0, 3], [0, 2, 1], [1, 2, 0]], result: 5 },
  { m: [[1], [5], [0], [2]], result: 6 },
  { m: [[1, 2, 3, 4, 5]], result: 15 },
  { m: [[2], [5], [10]], result: 17 },
  { m: [[4, 0, 1], [10, 7, 0], [0, 0, 0], [9, 1, 2]], result: 15 },
  { m: [[1]], result: 1 },
]

testCase.forEach(i => {
  let result = matrixElementsSum(i.m)
  console.log(`${i.m} => ${result} [${result == i.result ? '✔ Passed' : '❌ Fail'}]`)
})