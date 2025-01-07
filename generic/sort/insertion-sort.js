/**
 * @param {Array<number>} arr The input integer array to be sorted.
 * @return {Array<number>}
 */
export default function insertionSort(arr) {
	for (let i = 0; i < arr.length; i++) {
		const value = arr[i];
		let j = i - 1;
		while (j >= 0 && arr[j] > value) {
			arr[j + 1] = arr[j];
			j--;
		}

		if (j + 1 !== i) {
			arr[j + 1] = value;
		}
	}
	return arr;
}
