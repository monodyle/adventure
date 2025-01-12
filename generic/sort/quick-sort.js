/**
 * @param {Array<number>} arr The input integer array to be sorted.
 * @return {Array<number>}
 */
export default function quickSort (arr) {
  if (arr.length <= 1) {
    return arr
  }

  _sort(arr, 0, arr.length - 1)
  return arr
}

function _sort (arr, low, high) {
  if (low < high) {
    const pi = _partial(arr, low, high)
    _sort(arr, low, pi - 1)
    _sort(arr, pi + 1, high)
  }
}

function _partial (arr, low, high) {
  const pivot = arr[high]
  let i = low
  for (let j = low; j < high; j++) {
    if (arr[j] <= pivot) {
      ;[arr[i], arr[j]] = [arr[j], arr[i]]
      i++
    }
  }

  ;[arr[i], arr[high]] = [arr[high], arr[i]]
  return i
}
