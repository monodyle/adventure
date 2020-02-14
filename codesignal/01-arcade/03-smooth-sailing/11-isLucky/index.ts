/*
 * Problem:
 *  Ticket numbers usually consist of an even number of digits. A ticket number is considered lucky if the sum of the first half of the digits is equal to the sum of the second half.
 *  Given a ticket number `n`, determine if it's lucky or not.
 * Example:
 *  For `n = 1230`, the output should be `isLucky(n) = true`
 *  For `n = 239017`, the output should be `isLucky(n) = false`
 */

const REDUCE_CALLBACK = (sum, n) => sum + n

function isLucky(n: number): boolean {
  const length: number = Math.floor(String(n).length / 2)
  let fh: Array<number> = Math.floor(n / Math.pow(10, length)).toString().split('').map(Number)
  let sh: Array<number> = (n % Math.pow(10, length)).toString().split('').map(Number)
  return fh.reduce(REDUCE_CALLBACK, 0) === sh.reduce(REDUCE_CALLBACK, 0)
}

export { isLucky }