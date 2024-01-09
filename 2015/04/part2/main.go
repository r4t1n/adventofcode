package main

import (
	"crypto/md5"
	"fmt"
	"os"
	"strings"
)

func puzzle(input []byte) {
	prefix := "000000"
	var number int
	fmt.Println("Calculating...")
	for {
		checksum := md5.Sum([]byte(fmt.Sprintf("%s%d", input, number)))
		if strings.HasPrefix(fmt.Sprintf("%x", checksum), prefix) {
			break
		}
		number++
	}

	fmt.Printf("First number with hash prefix %s (%d digits): %d\n", prefix, len(prefix), number)
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
