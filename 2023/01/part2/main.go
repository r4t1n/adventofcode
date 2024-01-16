package main

import (
	"fmt"
	"os"
	"strings"
	"unicode"
)

var spelledOutDigits = [9]string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}

func puzzle(input []byte) {
	var twoDigits []int
	for _, line := range strings.Split(string(input), "\n") { // Iterate over each line
		if len(line) > 0 { // Check if line is not empty
			// The character iteration loop will always run before the line iteration loop, meaning that digits will always be before spelled out digits. Also theres some weird bug where the spelled out digits is out of order
			var validDigits []rune
			for _, char := range line { // Iterate over each character from the line
				if unicode.IsDigit(char) { // Check if character is a digit
					validDigits = append(validDigits, char) // Append digit to validDigits
				}
			}

			spelledOutDigitsToDigits := map[string]int{"one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7, "eight": 8, "nine": 9}
			for _, spelledOutDigit := range spelledOutDigits { // Iterate over each spelled out digit from the spelledOutDigits array
				if strings.Contains(line, spelledOutDigit) { // Check if the line contains a spelled out digit
					spelledOutDigitNumeric := spelledOutDigitsToDigits[spelledOutDigit] // Convert spelled out digit to digit
					validDigits = append(validDigits, rune(spelledOutDigitNumeric+'0')) // Append digit to validDigits (REMEMBER THE "+'0'" or else Unicode is gonna fuck you over with negative values)
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
