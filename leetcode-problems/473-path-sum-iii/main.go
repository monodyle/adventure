package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func main() {
	input := &TreeNode{
		Val: 10,
		Left: &TreeNode{
			Val: 5,
			Left: &TreeNode{
				Val:   3,
				Left:  &TreeNode{Val: 3},
				Right: &TreeNode{Val: -2}},
			Right: &TreeNode{
				Val:   2,
				Right: &TreeNode{Val: 1}},
		},
		Right: &TreeNode{
			Val:   -3,
			Right: &TreeNode{Val: 11},
		},
	}
	fmt.Printf("%#v => %#v", input, pathSum(input, 8))
}

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
