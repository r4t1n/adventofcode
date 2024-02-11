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

func puzzle(input []byte) {
	var length, width, height int
	var totalAreas int
	for _, line := range strings.Split(string(input), "\n") { // Iterate over each line from input
		if len(line) > 0 { // Check the line is not whitespace
			extractDimensions(string(line), &length, &width, &height)        // Extract the dimensions from the line
			surfaceArea := 2*length*width + 2*width*height + 2*height*length // Calculate the surface area (formula used from example)
			dimensions := []int{length, width, height}                       // Put the dimensions in a slice
			sort.Ints(dimensions)                                            // Sort the dimensions in ascending order
			slack := dimensions[0] * dimensions[1]                           // Calculate the slack (sum of the areas of the two smallest sides)
			totalArea := surfaceArea + slack                                 // Calculate the total area
			totalAreas += totalArea                                          // Add the total area for the line to the totalAreas variable
			fmt.Printf("Line: %s, length: %d, width: %d, height: %d, surface area: %d, slack: %d, total area: %d\n", line, length, width, height, surfaceArea, slack, totalArea)
		}
	}

	fmt.Println("\nSum:", totalAreas)
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
