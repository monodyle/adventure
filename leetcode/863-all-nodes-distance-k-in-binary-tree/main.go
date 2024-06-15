package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func main() {
	input := &TreeNode{
		Val: 3,
		Left: &TreeNode{
			Val: 5,
			Left: &TreeNode{
				Val: 6,
			},
			Right: &TreeNode{
				Val: 2,
				Left: &TreeNode{
					Val: 7,
				},
				Right: &TreeNode{
					Val: 4,
				},
			},
		},
		Right: &TreeNode{
			Val: 1,
			Left: &TreeNode{
				Val: 1,
			},
			Right: &TreeNode{
				Val: 8,
			},
		},
	}
	fmt.Printf("%#v => %#v", input, distanceK(input, &TreeNode{
		Val: 5,
	}, 2))
}

func distanceK(root *TreeNode, target *TreeNode, k int) []int {
	parents := map[*TreeNode]*TreeNode{}

	var dfs func(root, parent *TreeNode)
	dfs = func(root, parent *TreeNode) {
		if root == nil {
			return
		}
		parents[root] = parent
		dfs(root.Left, root)
		dfs(root.Right, root)
	}
	dfs(root, nil)

	q := []*TreeNode{target}
	seen := map[*TreeNode]bool{}
	seen[target] = true
	var dist int
	var result []int = []int{}

	for len(q) > 0 {
		if dist == k {
			for _, v := range q {
				result = append(result, v.Val)
			}
			return result
		}
		l := len(q)
		for i := 0; i < l; i++ {
			if _, ok := seen[q[i].Left]; !ok && q[i].Left != nil {
				seen[q[i].Left] = true
				q = append(q, q[i].Left)
			}
			if _, ok := seen[q[i].Right]; !ok && q[i].Right != nil {
				seen[q[i].Right] = true
				q = append(q, q[i].Right)
			}
			if _, ok := seen[parents[q[i]]]; !ok && parents[q[i]] != nil {
				seen[parents[q[i]]] = true
				q = append(q, parents[q[i]])
			}
		}
		q = q[l:]
		dist++
	}

	return []int{}
}
