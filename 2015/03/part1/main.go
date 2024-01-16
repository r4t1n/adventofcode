package main

import (
	"fmt"
	"os"
	"strconv"
	"unicode"
)

func puzzle(input []byte) {
	presents := make(map[string]int)
	var x int
	var y int
	for _, char := range input {
		if !unicode.IsSpace(rune(char)) { // Don't iterate over an empty character
			if char == '^' {
				y++
				fmt.Println("North")
			} else if char == 'v' {
				y--
				fmt.Println("South")
			} else if char == '>' {
				x++
				fmt.Println("East")
			} else if char == '<' {
				x--
				fmt.Println("West")
			} else {
				fmt.Printf("Character: %c is invalid (not a direction: ^, v, >, <)\n", char)
			}

			xString := strconv.Itoa(x)
			yString := strconv.Itoa(y)
			coordinate := xString + ", " + yString
			presents[coordinate] += 1

			fmt.Printf("Delivering present to coordinate: %s\n", coordinate)
		}
	}

	var housesWithPresents int
	for key, value := range presents {
		housesWithPresents++
		fmt.Printf("Coordinate: %s, presents: %d\n", key, value)
	}

	fmt.Printf("\nHouses with presents: %d\n", housesWithPresents)
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
