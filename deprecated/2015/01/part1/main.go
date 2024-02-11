package main

import (
	"fmt"
	"os"
	"unicode"
)

func puzzle(input []byte) {
	var floor int
	for _, char := range input {
		if !unicode.IsSpace(rune(char)) { // Don't iterate over an empty character
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

func main() {
	if len(os.Args) < 2 { // Check if input is provided
		fmt.Println("Please provide the input as the first argument (go run main.go <input>)")
		os.Exit(1)
	}

	inputFile := os.Args[1]              // Get the input file from the first argument
	input, err := os.ReadFile(inputFile) // Read the input from the input file
	if err != nil {
		fmt.Println("Error reading file:", err)
		os.Exit(1)
	}

	puzzle(input)
}
