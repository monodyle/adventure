package main

import "fmt"

/**
 * Problem: https://leetcode.com/problems/palindrome-linked-list/
 * References:
 *  - https://github.com/labuladong/fucking-algorithm/blob/english/data_structure/reverse_part_of_a_linked_list_via_recursion.md
 *  - https://labuladong.gitbook.io/algo-en/iv.-high-frequency-interview-problem/check_palindromic_linkedlist
 */

type ListNode struct {
	Val  int
	Next *ListNode
}

func main() {
	fmt.Println("Easy - Palindrome Linked List")
	head := ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 2,
				Next: &ListNode{
					Val:  1,
					Next: nil,
				},
			},
		},
	}

	head.print()
	fmt.Printf(" => %v \n", isPalindrome(&head))
}

func isPalindrome(head *ListNode) bool {
	if head.Next == nil {
		return true
	}

	slow, fast := head, head
	stack := []int{}

	for fast != nil && fast.Next != nil {
		stack = append(stack, slow.Val)
		slow = slow.Next
		fast = fast.Next.Next
	}

	if fast != nil {
		slow = slow.Next
	}

	for slow != nil {
		if slow.Val != stack[len(stack)-1] {
			return false
		}
		slow = slow.Next
		stack = stack[:len(stack)-1]
	}

	return true
}

func (list *ListNode) print() {
	if list == nil {
		fmt.Print("NULL")
		return
	}
	fmt.Printf("%v -> ", list.Val)
	list.Next.print()
}
