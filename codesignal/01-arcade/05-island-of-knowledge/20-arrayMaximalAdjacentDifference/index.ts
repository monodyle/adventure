/*
 * Problem: Given an array of integers, find the maximal absolute difference between any two of its adjacent elements.
 * Example: For `[2, 4, 1, 0]` the output should be `3`
 */

function arrayMaximalAdjacentDifference(input: number[]): number {
  return Math.max(...input.slice(1).map((v, i) => Math.abs(v - input[i])))
}

export { arrayMaximalAdjacentDifference }