/*
 * Problem: Some people are standing in a row in a park. There are trees between them which cannot be moved. Your task is to rearrange the people by their heights in a non-descending order without moving the trees. People can be very tall!
 * Example:
 *  For `a = [-1, 150, 190, 170, -1, -1, 160, 180]`
 *  the output should be `sortByHeight(a) = [-1, 150, 160, 170, -1, -1, 180, 190]`
 */

function sortByHeight(a: number[]): number[] {
  const normalSort: Array<number> = a.filter(i => i != -1).sort((a, b) => a - b)
  let flag: number = 0
  a.forEach(i => {
    if (i != -1) i = normalSort[flag++]
  })
  return a
}

export { sortByHeight }