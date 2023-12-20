package main

import (
	"fmt"
	"os"
	"path/filepath"
	"unicode"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Printf("Usage: go run %s.go <input>\n", filepath.Base(os.Args[0]))
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
		if !unicode.IsSpace(rune(char)) { // Don't iterate over a new line
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
