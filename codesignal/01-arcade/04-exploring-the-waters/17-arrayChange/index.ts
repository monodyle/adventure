/*
 * Problem: You are given an array of integers. On each move you are allowed to increase exactly one of its element by one. Find the minimal number of moves required to obtain a strictly increasing sequence from the input.
 * Example:
 *  For `[1, 1, 1]` the output should be `3`
 */

function arrayChange(inputArray: number[]): number {
  let sum: number = 0

  for (let i = 1; i < inputArray.length; i++) {
    if (inputArray[i] <= inputArray[i - 1]) {
      sum += inputArray[i - 1] - inputArray[i] + 1
      inputArray[i] = inputArray[i - 1] + 1
    }
  }

  return sum
}

export { arrayChange }