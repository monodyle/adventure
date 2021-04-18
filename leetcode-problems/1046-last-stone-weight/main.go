package main

import (
	"container/heap"
	"fmt"
)

type max_heap []int

func (h max_heap) Len() int            { return len(h) }
func (h max_heap) Less(i, j int) bool  { return h[i] > h[j] }
func (h max_heap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *max_heap) Push(x interface{}) { *h = append(*h, x.(int)) }
func (h *max_heap) Pop() interface{} {
	n := len(*h)
	x := (*h)[n-1]
	*h = (*h)[:n-1]
	return x
}

func MaxIntSlice(s []int) (int, int) {
	index := 0
	max := s[index]
	for i, v := range s {
		if v > max {
			index = i
			max = v
		}
	}
	return max, index
}

func lastStoneWeightBruteForce(stones []int) int {
	for {
		if len(stones) == 0 {
			return 0
		}
		if len(stones) == 1 {
			return stones[0]
		}
		y, i := MaxIntSlice(stones)
		stones = append(stones[:i], stones[i+1:]...)
		x, j := MaxIntSlice(stones)
		stones = append(stones[:j], stones[j+1:]...)
		if y != x {
			stones = append(stones, y-x)
		}
	}
}

func lastStoneWeight(stones []int) int {
	pq := max_heap(stones)
	heap.Init(&pq)
	for pq.Len() > 1 {
		heap.Push(&pq, heap.Pop(&pq).(int)-heap.Pop(&pq).(int))
	}
	return heap.Pop(&pq).(int)
}

func main() {
	input := []int{2, 7, 4, 1, 8, 1}
	fmt.Printf("Input: %#v => Output: %#v\n", input, lastStoneWeight(input))
	input = []int{1, 1}
	fmt.Printf("Input: %#v => Output: %#v\n", input, lastStoneWeightBruteForce(input))
}
