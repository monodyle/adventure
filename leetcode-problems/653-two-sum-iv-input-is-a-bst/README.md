# Two Sum IV - Input is a BST [Easy]

Problem: https://leetcode.com/problems/two-sum-iv-input-is-a-bst/

## Example

```go
func findTarget(root *TreeNode, k int) bool {
	m := make(map[int]int)
	var find func(node *TreeNode) bool
	find = func(node *TreeNode) bool {
		if node == nil {
			return false
		}
		if _, ok := m[k-node.Val]; ok {
			return ok
		}
		m[node.Val]++
		return find(node.Left) || find(node.Right)
	}
	return find(root)
}
```

## Submission Detail

```
422 / 422 test cases passed.
Status: Accepted
Runtime: 28 ms
Memory Usage: 7.3 MB
```
