/*
 * Problem: Given a string, find out if its characters can be rearranged to form a palindrome.
 * Example:
 *  For inputString = "aabb", the output should be palindromeRearranging(inputString) = true.
 *  We can rearrange "aabb" to make "abba", which is a palindrome.
 */

function palindromeRearranging(inputString: string): boolean {
  let unique: string[] = Array.from(new Set([...inputString.split("")]))
  let [even, odd] = unique.reduce((count, el) => ( count[((inputString.match(new RegExp(el, "g")) || []).length % 2 === 0) ? 0 : 1] += 1, count ), [0, 0])
  if (odd >= 2) return false
  return true
}

export { palindromeRearranging }