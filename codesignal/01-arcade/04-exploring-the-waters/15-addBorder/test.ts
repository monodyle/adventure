import { addBorder } from './index'
var array = require('lodash/array');

let testCase = [
  {input: ["abc", "ded"], result: ["*****", "*abc*", "*ded*", "*****"]},
  {input: ["a"], result: ["***", "*a*", "***"]},
  {input: ["aa", "**", "zz"], result: ["****", "*aa*", "****", "*zz*", "****"]},
  {input: ["abcde", "fghij", "klmno", "pqrst", "uvwxy"], result: ["*******", "*abcde*", "*fghij*", "*klmno*", "*pqrst*", "*uvwxy*", "*******"]},
  {input: ["wzy**"], result: ["*******", "*wzy***", "*******"]},
]

testCase.forEach(i => {
  let result = addBorder(i.input)
  let passed = array.difference(result, i.result).length === 0
  let debug = !passed ? ` - Want: ${i.result} | Given: ${result}` : ''
  console.log(`Input: ${i.input} [${passed ? '✔ Passed' : '❌ Fail'}${debug}]`)
})