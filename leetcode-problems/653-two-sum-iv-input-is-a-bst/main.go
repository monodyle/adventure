package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func main() {
	input := &TreeNode{
		Val: 5,
		Left: &TreeNode{
			Val: 3,
			Left: &TreeNode{
				Val: 2,
			},
			Right: &TreeNode{Val: 4},
		},
		Right: &TreeNode{
			Val: 6,
			Right: &TreeNode{
				Val: 7,
			},
		},
	}
	fmt.Printf("%#v => %#v\n", input, findTarget(input, 9))
}

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
