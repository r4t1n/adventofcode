package main

import (
	"fmt"
	"os"
	"strings"
	"unicode"
)

func main() {
	// Check if input is provided
	if len(os.Args) < 2 {
		fmt.Println("Usage: go run part1.go <input>")
		return
	}

	input_file := os.Args[1]              // Get the input file from the second argument
	input, err := os.ReadFile(input_file) // Read the file and store it's contents in input
	if err != nil {
		panic(err)
	}

	var input_two_digits []int
	for _, line := range strings.Split(string(input), "\n") { // Iterate over each line
		if len(line) > 0 { // Check if line is not empty
			var valid_digits []rune
			for _, char := range line { // Iterate over each character from the line
				if unicode.IsDigit(char) { // Check if character is a digit
					valid_digits = append(valid_digits, char) // Append digit to valid_digits
				}
			}

			if len(valid_digits) > 0 { // Check if there are valid digits in the line
				first_digit := int(valid_digits[0] - '0')
				last_digit := int(valid_digits[len(valid_digits)-1] - '0')

				// If the line contains only one digit, repeat it
				if len(valid_digits) == 1 {
					input_two_digits = append(input_two_digits, first_digit*10+first_digit)
				} else {
					input_two_digits = append(input_two_digits, first_digit*10+last_digit)
				}
				fmt.Printf("Line: %s, two-digit number: %d\n", line, first_digit*10+last_digit)
			}
		}
	}

	var sum int
	for _, value := range input_two_digits {
		sum += value
	}
	fmt.Println("\nSum:", sum)
}
