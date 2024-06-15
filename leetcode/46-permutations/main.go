package main

import "fmt"

/**
 * Problem: https://leetcode.com/problems/permutations/
 * References:
 *  - https://github.com/labuladong/fucking-algorithm/blob/english/think_like_computer/DetailsaboutBacktracking.md
 *  - https://labuladong.gitbook.io/algo-en/iii.-algorithmic-thinking/subset_permutation_combination#3-permutation
 */

func main() {
	fmt.Println("\nMedium - Permutations")
	nums := []int{1, 2, 3}
	fmt.Printf("#%v => %#v", nums, permute(nums))
}

func permute(nums []int) [][]int {
	if len(nums) == 0 {
		return nil
	}

	result := make([][]int, 0)
	backtrack(nums, []int{}, &result)
	return result
}

func backtrack(nums []int, prev []int, result *[][]int) {
	if len(nums) == 0 {
		*result = append(*result, prev)
		return
	}

	for i, num := range nums {
		backtrack(
			append(
				append([]int{}, nums[:i]...),
				nums[i+1:]...,
			),
			append(prev, num),
			result,
		)
	}
}
