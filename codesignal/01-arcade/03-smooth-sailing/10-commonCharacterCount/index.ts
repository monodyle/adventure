/*
 * Problem: Given two strings, find the number of common characters between them.
 * Example:
 *  For `s1 = "aabcc"` and `s2 = "adcaa"`, the output should be
 *  `commonCharacterCount(s1, s2) = 3`.
 *    Strings have `3` common characters - `2` "a"s and `1` "c".
 */

function commonCharacterCount(s1: string, s2: string): number {
  return s1.split('').filter(i => {
    let common = s2.includes(i)
    if (common) {
      s2 = s2.replace(new RegExp(i), '')
      return true
    }
    return false
  }).length
}

export { commonCharacterCount }