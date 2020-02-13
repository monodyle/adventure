import {adjacentElementsProduct} from './index'

let testCase = [
  {i: [3, 6, -2, -5, 7, 3], result: 21},
  {i: [-1, -2], result: 2},
  {i: [5, 1, 2, 3, 1, 4], result: 6},
  {i: [1, 2, 3, 0], result: 6},
  {i: [9, 5, 10, 2, 24, -1, -48], result: 50},
  {i: [5, 6, -4, 2, 3, 2, -23], result: 30},
  {i: [4, 1, 2, 3, 1, 5], result: 6},
  {i: [-23, 4, -3, 8, -12], result: -12},
  {i: [1, 0, 1, 0, 1000], result: 0},
]

testCase.forEach(i => {
  let result = adjacentElementsProduct(i.i)
  console.log(`Input ${i.i} => ${result} [${result == i.result ? '✔ Passed' : '❌ Fail'}]`)
})