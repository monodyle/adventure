package main

import (
	"strings"
)

func divideString(s string, k int, fill byte) []string {
	var ans []string
	for i := 0; i < len(s); i += k {
		var sub string
		if i+k < len(s) {
			sub = s[i : i+k]
		} else {
			sub = s[i:] + strings.Repeat(string(fill), k-len(s[i:]))
		}
		ans = append(ans, sub)
	}
	return ans
}
