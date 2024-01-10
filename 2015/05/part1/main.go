package main

import (
	"fmt"
	"os"
	"regexp"
	"strings"
)

func property2(str string) bool {
	for i := 1; i < len(str); i++ { // Iterate through each character in the string
		if str[i] == str[i-1] { // Check if the current character is equal to the character before it
			return true
		}
	}
	return false
}

func property3(str string) bool {
	for _, forbidden := range []string{"ab", "cd", "pq", "xy"} { // Check if string contains any string in forbidden strings
		if strings.Contains(str, forbidden) {
			return false
		}
	}
	return true
}

func puzzle(input []byte) {
	var niceStrings []string
	property1 := regexp.MustCompile("[aeiou].*[aeiou].*[aeiou]") // First rule for a nice string
	for _, line := range strings.Split(string(input), "\n") {    // Iterate over each line
		if len(line) > 0 { // Check if line is not empty
			match := property1.FindString(line)
			if len(match) > 0 { // Check for match in property1
				if property2(line) && property3(line) { // Check for true on property2 and property3
					niceStrings = append(niceStrings, line) // Append the nice string to niceStrings
				}
			}
		}
	}

	// Check if there are any nice strings
	if len(niceStrings) > 0 {
		var count int
		for _, str := range niceStrings { // Iterate over each nice string and add it to count
			count++
			fmt.Println(str)
		}

		fmt.Printf("\nNumber of nice strings: %d\n", count)
	} else {
		fmt.Println("No nice strings found in input")
	}
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
