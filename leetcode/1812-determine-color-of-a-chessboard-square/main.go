package main

import "fmt"

func main() {
	input := "a1"
	fmt.Printf("%#v => %#v\n", input, squareIsWhite(input))
}

func squareIsWhite(coordinates string) bool {
	return (int(coordinates[0])-96-int(coordinates[1]))%2 != 0
}
