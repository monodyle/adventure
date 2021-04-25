package main

import "fmt"

func main() {
	input := []int{7, 1, 5, 3, 6, 4}
	fmt.Printf("Input: %#v => Output: %#v\n", input, maxProfit(input))
}

func maxProfit(prices []int) int {
	l, h := prices[0], 0
	for _, p := range prices {
		l = min(p, l)
		h = max(h, p-l)
	}
	return h
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
