package main

import (
	"fmt"
	"os"
	"unicode"
)

func main() {
	if len(os.Args) < 2 { // Check if input is provided
		fmt.Println("Please provide the input as the first argument (go run main.go <input>)")
		os.Exit(1)
	}

	inputFile := os.Args[1]
	input, err := os.ReadFile(inputFile)
	if err != nil {
		fmt.Println("Error reading file:", err)
		os.Exit(1)
	}

	var floor, position, basementPosition int
	for _, char := range input {
		if !unicode.IsSpace(rune(char)) { // Don't iterate over an empty character
			if char == '(' {
				floor++
				position++
				fmt.Println("Up")
			} else if char == ')' {
				floor--
				position++
				fmt.Println("Down")
			} else {
				fmt.Printf("Character: %c is invalid (not a bracket)\n", char)
			}

			if floor == -1 && basementPosition == 0 {
				basementPosition = position
				fmt.Println("First at basement")
			}
		}
	}

	fmt.Printf("\nFirst basement position: %d\n", basementPosition)
}
