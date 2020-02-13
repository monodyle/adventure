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