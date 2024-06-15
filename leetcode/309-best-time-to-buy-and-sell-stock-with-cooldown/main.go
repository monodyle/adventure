package main

import "fmt"

func main() {
	input := []int{1, 2, 3, 0, 2}
	fmt.Printf("Input: %#v => Output: %#v\n", input, maxProfit(input))
}

func maxProfit(prices []int) int {
	if len(prices) < 2 {
		return 0
	}
	r, h, s := 0, -prices[0], 0
	for i := 1; i < len(prices); i++ {
		_s := s
		s = h + prices[i]
		h = max(r-prices[i], h)
		r = max(r, _s)
	}
	return max(r, s)
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
