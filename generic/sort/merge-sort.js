/**
 * @param {Array<number>} arr The input integer array to be sorted.
 * @return {Array<number>}
 */
export default function mergeSort (arr) {
  if (arr.length <= 1) {
    return arr
  }

  const mid = Math.floor(arr.length / 2)
  const left = arr.slice(0, mid)
  const right = arr.slice(mid)

  const sortedLeft = mergeSort(left)
  const sortedRight = mergeSort(right)

  return _merge(sortedLeft, sortedRight)
}

function _merge (a, b) {
  const merged = []

  let i = 0
  let j = 0
  while (i < a.length && j < b.length) {
    if (a[i] < b[j]) {
      merged.push(a[i])
      i++
    } else {
      merged.push(b[j])
      j++
    }
  }

  merged.push(...a.slice(i), ...b.slice(j))

  return merged
}
