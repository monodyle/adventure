import { areEquallyStrong } from './index'

let testCase = [
  {yl: 10, yr: 15, fl: 15, fr: 10, result: true},
  {yl: 15, yr: 10, fl: 15, fr: 10, result: true},
  {yl: 15, yr: 10, fl: 15, fr: 9, result: false},
  {yl: 10, yr: 5, fl: 5, fr: 10, result: true},
  {yl: 10, yr: 15, fl: 5, fr: 20, result: false},
  {yl: 10, yr: 20, fl: 10, fr: 20, result: true},
  {yl: 5, yr: 20, fl: 20, fr: 5, result: true},
  {yl: 20, yr: 15, fl: 5, fr: 20, result: false},
  {yl: 5, yr: 10, fl: 5, fr: 10, result: true},
  {yl: 1, yr: 10, fl: 10, fr: 0, result: false},
  {yl: 5, yr: 5, fl: 10, fr: 10, result: false},
  {yl: 10, yr: 5, fl: 10, fr: 6, result: false},
  {yl: 1, yr: 1, fl: 1, fr: 1, result: true},
  {yl: 0, yr: 10, fl: 10, fr: 0, result: true},
]

testCase.forEach(i => {
  let result = areEquallyStrong(i.yl, i.yr, i.fl, i.fr)
  let passed = result === i.result
  let debug = !passed ? ` - Want: ${i.result} | Given: ${result}` : ''
  console.log(`Input: ${i.yl} ${i.yr} ${i.fl} ${i.fr} [${passed ? '✔ Passed' : '❌ Fail'}${debug}]`)
})