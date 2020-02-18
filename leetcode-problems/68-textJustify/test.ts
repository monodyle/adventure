import { textJustification } from './index'

let testCase = [
  {
    input: ["This",
      "is",
      "an",
      "example",
      "of",
      "text",
      "justification."],
    l: 16,
    result:
      ["This    is    an",
        "example  of text",
        "justification.  "]
  },
  {
    input: ["Two",
      "words."],
    l: 11,
    result: ["Two words. "]
  },
  {
    input: ["Two",
      "words."],
    l: 10,
    result: ["Two words."]
  },
  {
    input: ["a",
      "b",
      "c",
      "longword"],
    l: 9,
    result: ["a   b   c",
      "longword "]
  },
  {
    input: ["Two",
      "words."],
    l: 9,
    result: ["Two      ",
      "words.   "]
  },
  {
    input: ["Looks",
      "like",
      "it",
      "can",
      "be",
      "a",
      "tricky",
      "test"],
    l: 25,
    result: ["Looks  like  it  can be a",
      "tricky test              "]
  },
  {
    input: ["a",
      "b",
      "b",
      "longword"],
    l: 8,
    result: ["a   b  b",
      "longword"]
  },
  {
    input: ["vba",
      "gaff",
      "ye",
      "gs",
      "cthj",
      "hf",
      "vetjj",
      "jm",
      "k",
      "f",
      "cgbf",
      "zzz"],
    l: 8,
    result: ["vba gaff",
      "ye    gs",
      "cthj  hf",
      "vetjj jm",
      "k f cgbf",
      "zzz     "]
  },
  {
    input: ["Given",
      "an",
      "array",
      "of",
      "words",
      "and",
      "a",
      "length"],
    l: 9,
    result: ["Given  an",
      "array  of",
      "words and",
      "a length "]
  },
  {
    input: ["Extra",
      "spaces",
      "between",
      "words",
      "should",
      "be",
      "distributed",
      "as",
      "evenly",
      "as",
      "possible"],
    l: 20,
    result: ["Extra spaces between",
      "words    should   be",
      "distributed       as",
      "evenly as possible  "]
  },
  {
    input: [""],
    l: 2,
    result: ["  "]
  },
  {
    input: ["a"],
    l: 1,
    result: ["a"]
  },
  {
    input: ["a"],
    l: 2,
    result: ["a "]
  },
  {
    input: ["a",
      "b",
      "c",
      "d",
      "e"],
    l: 1,
    result: ["a",
      "b",
      "c",
      "d",
      "e"]
  },
  {
    input: ["a",
      "b",
      "c",
      "d",
      "e"],
    l: 3,
    result: ["a b",
      "c d",
      "e  "]
  },
]

testCase.forEach(i => {
  let result = textJustification(i.input, i.l)
  let passed: boolean = result.toString() === i.result.toString()
  let debug: string = !passed ? ` - Want: ${i.result} | Given: ${result}` : ''
  console.log(`Input: ${i.input} => ${result} [${passed ? '✔ Passed' : '❌ Fail'}${debug}]`)
})