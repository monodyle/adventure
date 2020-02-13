import { centuryFromYear } from './index'

let testCase = [
  {year: 1905, result: 20},
  {year: 1700, result: 17},
  {year: 1988, result: 20},
  {year: 2000, result: 20},
  {year: 2001, result: 21},
  {year: 200, result: 2},
  {year: 374, result: 4},
  {year: 45, result: 1},
  {year: 8, result: 1},
]

testCase.forEach(i => {
  let result = centuryFromYear(i.year)
  console.log(`${i.year} => ${result} [${result == i.result ? '✔ Passed' : '❌ Fail'}]`)
})