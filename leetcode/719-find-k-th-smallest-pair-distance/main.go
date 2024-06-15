package main

import (
	"fmt"
	"sort"
)

func main() {
	input := []int{0, 1, 1, 2, 2, 2, 4, 5}
	fmt.Printf("%#v => %#v\n", input, smallestDistancePair(input, 5))
}

func smallestDistancePair(nums []int, k int) int {
	n := len(nums)
	sort.Ints(nums)

	low := nums[1] - nums[0]
	for i := 2; i < n; i++ {
		low = min(low, nums[i]-nums[i-1])
	}

	high := nums[n-1] - nums[0]
	for low < high {
		mid := low + (high-low)/2
		if count(nums, mid) < k {
			low = mid + 1
		} else {
			high = mid
		}
	}

	return low
}

func count(nums []int, mid int) int {
	n := len(nums)
	res := 0
	i, j := 0, 1
	for j < n {
		if nums[j]-nums[i] <= mid {
			res += j - i
			j++
		} else {
			i++
		}
	}
	return res
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
