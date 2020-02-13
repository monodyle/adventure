import { checkPalindrome } from './index'

let testCase = [
  {s: "aabaa", result: true},
  {s: "abac", result: false},
  {s: "a", result: true},
  {s: "az", result: false},
  {s: "abacaba", result: true},
  {s: "z", result: true},
  {s: "aaabaaaa", result: false},
  {s: "zzzazzazz", result: false},
  {s: "hlbeeykoqqqqokyeeblh", result: true},
  {s: "hlbeeykoqqqokyeeblh", result: true},
]

testCase.forEach(i => {
  let result = checkPalindrome(i.s)
  console.log(`${i.s} [${result == i.result ? '✔ Passed' : '❌ Fail'}]`)
})