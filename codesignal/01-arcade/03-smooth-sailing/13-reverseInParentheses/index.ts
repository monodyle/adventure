/*
 * Problem:
 *  Write a function that reverses characters in (possibly nested) parentheses in the input string.
 * Input strings will always be well-formed with matching `()`s.
 * Example:
 *  For `"(bar)"`, the output should be `"rab"`
 *  For `"foo(bar)baz"`, the output should be `"foorabbaz"`
 *  For `"foo(bar)baz(blim)"`, the output should be `"foorabbazmilb"`
 *  For `"foo(bar(baz))blim"`, the output should be `"foobazrabblim"`
 *  Because `"foo(bar(baz))blim"` becomes `"foo(barzab)blim"` and then `"foobazrabblim"`
 */

function reverseInParentheses(input: string): string {
  const pattern: RegExp = /\(([^()]*)\)/i
  while (pattern.test(input)) {
    let sub = pattern.exec(input)[1].split("").reverse().join("")
    input = input.replace(pattern, sub)
  }
  return input
}

export { reverseInParentheses }