package main

import "fmt"

func main() {
	n := 1
	fmt.Printf("%#v => %#v", n, knightDialer(n))
}

const m = 1000000007

func knightDialer(n int) int {
	c := []int{1, 1, 1, 1, 1, 1, 1, 1, 1, 1}
	for i := 1; i < n; i++ {
		n := make([]int, 10)
		n[0] = (c[4] + c[6]) % m
		n[1] = (c[6] + c[8]) % m
		n[2] = (c[7] + c[9]) % m
		n[3] = (c[4] + c[8]) % m
		n[4] = (c[3] + c[9] + c[0]) % m
		n[5] = 0
		n[6] = (c[1] + c[7] + c[0]) % m
		n[7] = (c[2] + c[6]) % m
		n[8] = (c[1] + c[3]) % m
		n[9] = (c[2] + c[4]) % m
		c = n
	}
	s := 0
	for _, v := range c {
		s += v
	}
	return s % m
}
