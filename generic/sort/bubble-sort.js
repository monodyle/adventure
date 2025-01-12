/**
 * @param {Array<number>} arr The input integer array to be sorted.
 * @return {Array<number>}
 */
export default function bubbleSort(arr) {
	let isSwapped;
	for (let i = 0; i < arr.length; i++) {
		isSwapped = true;
		for (let j = 0; j < arr.length - i - 1; j++) {
			if (arr[j] > arr[j + 1]) {
				[arr[j], arr[j + 1]] = [arr[j + 1], arr[j]];
				isSwapped = false;
			}
		}
		if (isSwapped) {
			break;
		}
	}

	return arr;
}
