import { allLongestStrings } from './index'

let testCase = [
  {input: ["aba", "aa", "ad", "vcd", "aba"], result: ["aba", "vcd", "aba"]},
  {input: ["aa"], result: ["aa"]},
  {input: ["abc", "eeee", "abcd", "dcd"], result: ["eeee", "abcd"]},
  {input: ["a", "abc", "cbd", "zzzzzz", "a", "abcdef", "asasa", "aaaaaa"], result: ["zzzzzz", "abcdef", "aaaaaa"]},
  {input: ["enyky", "benyky", "yely", "varennyky"], result: ["varennyky"]},
  {input: ["abacaba", "abacab", "abac", "xxxxxx"], result: ["abacaba"]},
  {input: ["young", "yooooooung", "hot", "or", "not", "come", "on", "fire", "water", "watermelon"], result: ["yooooooung", "watermelon"]},
  {input: ["onsfnib", "aokbcwthc", "jrfcw"], result: ["aokbcwthc"]},
  {input: ["lbgwyqkry"], result: ["lbgwyqkry"]},
  {input: ["i"], result: ["i"]},
]

testCase.forEach(i => {
  let result = allLongestStrings(i.input)
  let passed = result.every(e => i.result.includes(e)) && i.result.every((e) => result.includes(e))
  let debug = !passed ? ` - Want: ${i.result} | Given: ${result}` : ''
  console.log(`Input: ${i.input} [${passed ? '✔ Passed' : '❌ Fail'}${debug}]`)
})