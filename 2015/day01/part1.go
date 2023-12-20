package main

import (
	"fmt"
	"os"
	"unicode"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: go run part1.go <input>")
		os.Exit(1)
	}

	inputFile := os.Args[1]
	input, err := os.ReadFile(inputFile)
	if err != nil {
		fmt.Println("Error reading file:", err)
		os.Exit(1)
	}

	var floor int
	for _, char := range input {
		if !unicode.IsSpace(rune(char)) {
			if char == '(' {
				floor++
				fmt.Println("Up")
			} else if char == ')' {
				floor--
				fmt.Println("Down")
			} else {
				fmt.Printf("Character: %c is invalid (not a bracket)\n", char)
			}
		}
	}

	fmt.Printf("\nFinal floor: %d\n", floor)
}
