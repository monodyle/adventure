/*
 * Problem:
 *  In the popular Minesweeper game you have a board with some mines and those cells that don't contain a mine have a number in it that indicates the total number of mines in the neighboring cells.
 *  Starting off with some arrangement of mines we want to create a Minesweeper game setup.
 * Example:
 *  For [[true, false, false],
 *       [false, true, false],
 *       [false, false, false]]
 *  the output should be [[1, 2, 1],
 *                        [2, 1, 1],
 *                        [1, 1, 1]]
 *  Check out the image below for better understanding:
 *  ![example](./example.png)
 */

const directions: Array<[number, number]> = [
  [ 1,-1], [ 1, 0], [ 1, 1],
  [ 0,-1],          [ 0, 1],
  [-1,-1], [-1, 0], [-1, 1]
]

function minesweeper(matrix: boolean[][]): number[][] {
  return matrix.map((row, y) =>
    row.map((_, x) =>
      directions.reduce(
        (count, i) => count += (matrix[y + i[0]] && matrix[y + i[0]][x + i[1]]) ? 1 : 0, 0
      )
    )
  )
}

export { minesweeper }