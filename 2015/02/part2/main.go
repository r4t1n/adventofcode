package main

import (
	"fmt"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

func extractDimensions(line string, length, width, height *int) {
	pattern := regexp.MustCompile(`([0-9]+)x([0-9]+)x([0-9]+)`) // Nice static fucking pattern :3
	matches := pattern.FindAllStringSubmatch(line, 1)

	for _, match := range matches {
		*length, _ = strconv.Atoi(match[1])
		*width, _ = strconv.Atoi(match[2])
		*height, _ = strconv.Atoi(match[3])
	}
}

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

	var length, width, height int
	var ribbonSum int
	for _, line := range strings.Split(string(input), "\n") { // Iterate over each line from input
		if len(line) > 0 { // Check the line is not whitespace
			extractDimensions(string(line), &length, &width, &height)               // Extract the dimensions from the line
			dimensions := []int{length, width, height}                              // Put the dimensions in a slice
			sort.Ints(dimensions)                                                   // Sort the dimensions in ascending order
			ribbon := dimensions[0] + dimensions[0] + dimensions[1] + dimensions[1] // Calculate the ribbon size (sum of two smallest dimensions twice)
			bow := dimensions[0] * dimensions[1] * dimensions[2]                    // Calculate the bow size (product of all dimensions)
			totalRibbon := ribbon + bow                                             // Calculate the total ribbon size
			ribbonSum += totalRibbon                                                // Add the total ribbon size to the ribbon sum
			fmt.Printf("Line: %s, length: %d, width: %d, height: %d, ribbon: %d, bow: %d, total ribbon: %d\n", line, length, width, height, ribbon, bow, totalRibbon)
		}
	}

	fmt.Println("\nSum:", ribbonSum)
}
