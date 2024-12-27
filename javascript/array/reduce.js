/**
 * @template T, U
 * @param {(previousValue: U, currentValue: T, currentIndex: number, array: T[]) => U} callbackFn
 * @param {U} [initialValue]
 * @return {Array<U>}
 */
Array.prototype.myReduce = function (callbackFn, initialValue) {
  // expect to implement same behavior as Array.prototype.reduce

  // the difference of reduce with other iterable method of array
  // is reduce accept 2nd parameter as an initial value, which
  // used to accumulate til the result

  // so what happens if I let initial value undefined?
  // play with browser console:
  // empty array & undefined initial value:
  // > [].reduce(() => 0)
  // < Uncaught TypeError: Reduce of empty array with no initial value
  // array with 1 item & undefined initial value:
  // > [1].reduce(() => 0)
  // < 1 // look like first item
  // array with 1 item & with initial value:
  // > [1].reduce(() => 0, 5)
  // < 0 // look like the returning of the accumulation function
  // array with more items & undefined initial value:
  // > [1, 2, 3].reduce(() => 0)
  // < 0 // look like the returning of the accumulation function
  // array with more items & with initial value:
  // > [1, 2, 3].reduce(() => 0, 5)
  // < 0 // look like the returning of the accumulation function

  const noInitialValue = initialValue === undefined
  const l = this.length
  if (noInitialValue && l === 0) {
    throw new Error("Reduce of empty array with no initial value")
  }

  const k = noInitialValue ? 1 : 0
  let accumulator = noInitialValue ? this[0] : initialValue

  for (let i = k; i < l; i++) {
    if (Object.hasOwn(this, i)) {
      accumulator = callbackFn(accumulator, this[i], i, this)
    }
  }

  return accumulator
};
