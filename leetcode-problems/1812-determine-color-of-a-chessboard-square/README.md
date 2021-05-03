# Determine Color of a Chessboard Square [Easy]

Problem: https://leetcode.com/problems/determine-color-of-a-chessboard-square/

## Idea

I have to discussion with my friends and founded esiest solution is switch case:

```rust
match coordinates {
  "a1" => false,
  "a2" => true,
  "a3" => false,
  ...
  "h3" => true,
  ...
}
```

LOL, jk. I can quickly see that the rule of chessboard is `a1` always a black
square. So if we mapping alphabet char to number start from 1, a position have
sum of row and col is even number is a black one.

e.g:

- b5: b = 2; b + 5 = 7 (odd) => white square
- e7: e = 5; e + 7 = 12 (even) => black square

## Example

```go
func squareIsWhite(coordinates string) bool {
	return (int(coordinates[0])-96-int(coordinates[1]))%2 != 0
}
```

## Submission Detail

```
203 / 203 test cases passed.
Status: Accepted
Runtime: 0 ms
Memory Usage: 1.9 MB
```
