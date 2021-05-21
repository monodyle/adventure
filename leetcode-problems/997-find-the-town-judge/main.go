package main

import "fmt"

func main() {
	input := [][]int{{1, 2}}
	n := 2
	fmt.Printf("%#v => %#v", input, findJudge(n, input))
}

func findJudge(n int, trust [][]int) int {
	peoples := make([]int, n+1)
	trusted := make([]bool, n+1)
	for _, t := range trust {
		peoples[t[1]]++
		trusted[t[0]] = true
	}
	for i := 1; i <= n; i++ {
		if peoples[i] == n-1 && !trusted[i] {
			return i
		}
	}
	return -1
}
