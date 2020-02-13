function adjacentElementsProduct(input: number[]): number {
  let largest = input[0] * input[1]
  input.forEach((i, k) => {
    if (k != 0)
      largest = Math.max(largest, i * input[k - 1])
  })
  return largest
}

export {adjacentElementsProduct}