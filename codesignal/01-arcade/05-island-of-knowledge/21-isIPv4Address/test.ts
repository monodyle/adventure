import { isIPv4Address } from './index'

let testCase = [
  {input: "172.16.254.1", result: true},
  {input: "172.316.254.1", result: false},
  {input: ".254.255.0", result: false},
  {input: "1.1.1.1a", result: false},
  {input: "1", result: false},
  {input: "0.254.255.0", result: true},
  {input: "1.23.256.255.", result: false},
  {input: "1.23.256..", result: false},
  {input: "0..1.0", result: false},
  {input: "35..36.9.9.0", result: false},
  {input: "1.1.1.1.1", result: false},
  {input: "1.256.1.1", result: false},
  {input: "a0.1.1.1", result: false},
  {input: "0.1.1.256", result: false},
  {input: "129380129831213981.255.255.255", result: false},
  {input: "255.255.255.255abcdekjhf", result: false},
  {input: "7283728", result: false},
  {input: "0..1.0.0", result: false},
]

testCase.forEach(i => {
  let result = isIPv4Address(i.input)
  let passed = result === i.result
  let debug = !passed ? ` - Want: ${i.result} | Given: ${result}` : ''
  console.log(`Input: ${i.input} => ${result} [${passed ? '✔ Passed' : '❌ Fail'}${debug}]`)
})