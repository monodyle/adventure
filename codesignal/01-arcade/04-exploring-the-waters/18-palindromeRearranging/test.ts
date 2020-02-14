import { palindromeRearranging } from './index'

let testCase = [
  {input: "aabb", result: true},
  {input: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaabc", result: false},
  {input: "abbcabb", result: true},
  {input: "zyyzzzzz", result: true},
  {input: "z", result: true},
  {input: "zaa", result: true},
  {input: "abca", result: false},
  {input: "abcad", result: false},
  {input: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbccccaaaaaaaaaaaaa", result: false},
  {input: "abdhuierf", result: false},
]

testCase.forEach(i => {
  let result = palindromeRearranging(i.input)
  let passed = result === i.result
  let debug = !passed ? ` - Want: ${i.result} | Given: ${result}` : ''
  console.log(`Input: ${i.input} [${passed ? '✔ Passed' : '❌ Fail'}${debug}]`)
})