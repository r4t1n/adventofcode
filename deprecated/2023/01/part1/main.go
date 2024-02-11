package main

import (
	"fmt"
	"os"
	"strings"
	"unicode"
)

func puzzle(input []byte) {

	var twoDigits []int
	for _, line := range strings.Split(string(input), "\n") { // Iterate over each line
		if len(line) > 0 { // Check if line is not empty
			var validDigits []rune
			for _, char := range line { // Iterate over each character from the line
				if unicode.IsDigit(char) { // Check if character is a digit
					validDigits = append(validDigits, char) // Append digit to validDigits
				}
			}

			if len(validDigits) > 0 { // Check if there are valid digits
				firstDigit := int(validDigits[0] - '0')
				lastDigit := int(validDigits[len(validDigits)-1] - '0')
				if len(validDigits) == 1 { // If the line contains only one digit then repeat it
					twoDigits = append(twoDigits, firstDigit*10+firstDigit)
				} else {
					twoDigits = append(twoDigits, firstDigit*10+lastDigit)
				}
				fmt.Printf("Line: %s, two-digit number: %d\n", line, firstDigit*10+lastDigit)
			}
		}
	}

	var sum int
	for _, twoDigit := range twoDigits {
		sum += twoDigit
	}
	fmt.Println("\nSum:", sum)
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
