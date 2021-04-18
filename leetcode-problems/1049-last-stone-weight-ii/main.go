package main

import (
	"fmt"
)

func main() {
	input := []int{2, 7, 4, 1, 8, 1}
	fmt.Printf("Input: %#v => Output: %#v\n", input, lastStoneWeightII(input))
}

func lastStoneWeightII(stones []int) int {
	total_weight := 0
	for _, weight := range stones {
		total_weight += weight
	}
	half := total_weight / 2
	dynamic := make([]int, half+1)
	dynamic[0] = 1
	for _, stone := range stones {
		for x := half; x >= stone; x-- {
			dynamic[x] |= dynamic[x-stone]
		}
	}
	result := total_weight
	for x := 0; x <= half; x++ {
		result = min(result, total_weight-dynamic[x]*x*2)
	}
	return result
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
