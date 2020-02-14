import { reverseInParentheses } from './index'

let testCase = [
  {input: "(bar)", result: "rab"},
  {input: "foo(bar)baz", result: "foorabbaz"},
  {input: "foo(bar)baz(blim)", result: "foorabbazmilb"},
  {input: "foo(bar(baz))blim", result: "foobazrabblim"},
  {input: "", result: ""},
  {input: "()", result: ""},
  {input: "(abc)d(efg)", result: "cbadgfe"},
]

testCase.forEach(i => {
  let result = reverseInParentheses(i.input)
  let passed = result === i.result
  let debug = !passed ? ` - Want: ${i.result} | Given: ${result}` : ''
  console.log(`Input: ${i.input} [${passed ? '✔ Passed' : '❌ Fail'}${debug}]`)
})