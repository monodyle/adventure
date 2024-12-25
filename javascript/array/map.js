/**
 * @template T, U
 * @param { (value: T, index: number, array: Array<T>) => U } callbackFn
 * @param {any} [thisArg]
 * @return {Array<U>}
 */
Array.prototype.myMap = function (callbackFn, thisArg) {
	const results = new Array(this.length);
	for (let i = 0; i < this.length; i++) {
		if (!Object.hasOwn(this, i)) {
			// check the presence of index in array, e.g: `Object.hasOwn(['a', 'b', , 'd'], 2) === false`
			continue;
		}

		results[i] = callbackFn.call(thisArg, this[i], i, this);
	}
	return results;
};
