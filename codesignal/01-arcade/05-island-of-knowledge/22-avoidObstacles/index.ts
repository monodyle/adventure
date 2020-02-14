/*
 * Problem:
 *  You are given an array of integers representing coordinates of obstacles situated on a straight line.
 *  Assume that you are jumping from the point with coordinate 0 to the right. You are allowed only to make jumps of the same length represented by some integer.
 *  Find the minimal length of the jump enough to avoid all the obstacles.
 * Example:
 *  For `[5, 3, 6, 7, 9], the output should be `4`
 *  Check out the image below for better understanding:
 *  ![example](./example.png)
 */

const avoidObstacles = (input: number[]): number => {
  for (let n: number = 1;; n++) if (input.every( v => v % n )) return n
}

export { avoidObstacles }