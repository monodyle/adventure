/*
 * Problem: Below we will define an `n`-interesting polygon. Your task is to find the area of a polygon for a given `n`.
 * A `1`-interesting polygon is just a square with a side of length `1`.
 * An `n`-interesting polygon is obtained by taking the `n - 1`-interesting polygon and appending `1`-interesting polygons to its rim, side by side. You can see the `1`-, `2`-, `3`- and `4`-interesting polygons in the picture below.
 *
 * ![area](./area.png)
 */

const INCS_SQUARE_EACH_ROUND = 4

function shapeArea(n: number): number {
  let area = 1
  let increase = 0;
  for (let i = 2; i <= n; i++) {
    increase += INCS_SQUARE_EACH_ROUND
    area += increase
  }
  return area
}

export {shapeArea}