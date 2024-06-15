# Permutations [Medium]

Problem: https://leetcode.com/problems/permutations/

# 1. Idea

Backtracking. Just traverse the tree from root to leaves and record the numbers
on the paths, and we will gget all the permutations.

Example: The tree start with [ ] node (root) and have 3 branches: [1], [2], [3].
Pick up branch [1], the lost number is `2` and `3`, so the child branches are
them. Keep pick up branch [2], the lost number from root to this node is 3, so
it is. No more lost number, [3] is leaf, we have [1, 2, 3]. Keep going with
other branches which lost some number.

# 2. Step by Step

Permute make a backtrack with init numbers.

Backtrack take left number(s), previous nodes for each number left from previous
backtrack. If there no number left, return result.

# 3. Example

```go
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
```

# 4. Submission detail

```
25 / 25 test cases passed.
Status: Accepted
Runtime: 0 ms
Memory Usage: 2.7 MB
```
