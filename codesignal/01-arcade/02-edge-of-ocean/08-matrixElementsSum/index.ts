function matrixElementsSum(matrix: number[][]): number {
  let totalCost = 0
  let hautedCol = []
  matrix.forEach((row) => {
    row.forEach((price, pos) => {
      let hauted = hautedCol.includes(pos)
      if (price == 0 && !hauted) hautedCol.push(pos)
      if (!hauted) totalCost += price
    })
  })
  return totalCost
}

export { matrixElementsSum }