# Path Sum III [Medium]

Problem: https://leetcode.com/problems/path-sum-iii/

## Example

```go
func pathSum(root *TreeNode, targetSum int) int {
	if root == nil {
		return 0
	}
	var find func(node *TreeNode, sum int) int
	find = func(node *TreeNode, sum int) int {
		count := 0
		if node == nil {
			return 0
		}
		if node.Val == sum {
			count++
		}
		return count + find(node.Left, sum-node.Val) + find(node.Right, sum-node.Val)
	}
	return find(root, targetSum) + pathSum(root.Left, targetSum) + pathSum(root.Right, targetSum)
}
```

## Submission Detail

```
126 / 126 test cases passed.
Status: Accepted
Runtime: 16 ms
Memory Usage: 4.4 MB
```
