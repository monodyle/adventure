/**
 * @template T
 * @param { (value: T, index: number, array: Array<T>) => boolean } callbackFn
 * @param {any} [thisArg]
 * @return {Array<T>}
 */
Array.prototype.myFilter = function (callbackFn, thisArg) {
	const results = [];
	for (let i = 0; i < this.length; i++) {
		const value = this[i];
		if (callbackFn.call(thisArg, value, i, this)) {
			results.push(value);
		}
	}
	return results;
};
