/*
 * Problem: Given an array of integers, find the pair of adjacent elements that has the largest product and return that product.
 * Example:
 *  For `inputArray = [3, 6, -2, -5, 7, 3]`, the output should be `adjacentElementsProduct(inputArray) = 21`.
 *  `7` and `3` produce the largest product.
 */

function adjacentElementsProduct(input: number[]): number {
  let largest = input[0] * input[1]
  input.forEach((i, k) => {
    if (k != 0)
      largest = Math.max(largest, i * input[k - 1])
  })
  return largest
}

export {adjacentElementsProduct}