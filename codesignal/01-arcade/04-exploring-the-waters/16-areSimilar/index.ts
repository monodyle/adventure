/*
 * Problem:
 *  Two arrays are called similar if one can be obtained from another by swapping at most one pair of elements in one of the arrays.
 *  Given two arrays `a` and `b`, check whether they are similar.
 * Example:
 *  For `[1, 2, 3]` and `[1, 2, 3]`, the output should be `true`
 *    The arrays are equal, no need to swap any elements.
 *  For `[1, 2, 3]` and `[2, 1, 3]`, the output should be `true`
 *    We can obtain b from a by swapping 2 and 1 in b.
 *  For `[1, 2, 2]` and `[2, 1, 1]`, the output should be `false`
 *    Any swap of any two elements either in a or in b won't make a and b equal.
 */

function areSimilar(a: number[], b: number[]): boolean {
  if (a.toString() === b.toString()) return true

  let c: number[] =  a.filter((e, i) => e != b[i]),
      d: number[] = b.filter((e, i) => e != a[i])

  return c.length === 0 || (c.length === 2 && c.toString() === d.reverse().toString())
}

export { areSimilar }