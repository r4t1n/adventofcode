package main

import (
	"fmt"
	"os"
	"regexp"
	"strings"
)

func extractGameData(line string) {
	pattern := regexp.MustCompile(`Game ([0-9]+): *`) // Define the pattern for each line
	matches := pattern.FindAllStringSubmatch(line, 1) // Find the matches from the pattern

	for _, match := range matches { // Iterate over each match
		if len(match) > 1 { // Check if there are more than one match
			gameNumber := match[1] // The game number will always be the first match
			fmt.Println(gameNumber)
		}
	}
}

func main() {
	if len(os.Args) < 2 { // Check if input is provided
		fmt.Println("Please provide the input as the first argument (go run main.go <input>)")
		os.Exit(1)
	}

	inputFile := os.Args[1]              // Get the input file from the second argument
	input, err := os.ReadFile(inputFile) // Read the file and store it's contents in input
	if err != nil {
		fmt.Println("Error reading file:", err)
		os.Exit(1)
	}

	for _, line := range strings.Split(string(input), "\n") {
		if len(line) > 0 {
			extractGameData(line)
		}
	}
}
