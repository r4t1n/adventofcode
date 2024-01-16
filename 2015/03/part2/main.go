package main

import (
	"fmt"
	"os"
	"strconv"
	"unicode"
)

func puzzle(input []byte) {
	robotPresents := make(map[string]int)
	santaPresents := make(map[string]int)
	var robotTurn bool
	var robotX int
	var robotY int
	var santaX int
	var santaY int
	for _, char := range input {
		if !unicode.IsSpace(rune(char)) { // Don't iterate over an empty character
			if char == '^' {
				if !robotTurn {
					santaY++
					fmt.Println("Santa north")
				} else {
					robotY++
					fmt.Println("Robot north")
				}
			} else if char == 'v' {
				if !robotTurn {
					santaY--
					fmt.Println("Santa south")
				} else {
					robotY--
					fmt.Println("Robot south")
				}
			} else if char == '>' {
				if !robotTurn {
					santaX++
					fmt.Println("Santa east")
				} else {
					robotX++
					fmt.Println("Robot east")
				}
			} else if char == '<' {
				if !robotTurn {
					santaX--
					fmt.Println("Santa west")
				} else {
					robotX--
					fmt.Println("Robot west")
				}
			} else {
				fmt.Printf("Character: %c is invalid (not a direction: ^, v, >, <)\n", char)
			}

			// Set Santa and Robot specific map values
			var coordinate string
			if !robotTurn {
				xString := strconv.Itoa(santaX)
				yString := strconv.Itoa(santaY)
				coordinate = xString + ", " + yString
				santaPresents[coordinate] += 1
			} else {
				xString := strconv.Itoa(robotX)
				yString := strconv.Itoa(robotY)
				coordinate = xString + ", " + yString
				robotPresents[coordinate] += 1
			}

			robotTurn = !robotTurn // Flip the robotTurn bool
			fmt.Printf("Delivering present to coordinate: %s\n", coordinate)
		}
	}

	// Since we have two maps sometimes the coordinates overlap giving duplicate coordinates, to solve this we put both maps into one map
	presents := make(map[string]int)
	var housesWithPresents int
	fmt.Println("Adding Santa presents to presents")
	for key, value := range santaPresents {
		presents[key] += value
	}
	fmt.Println("Adding Robot presents to presents")
	for key, value := range robotPresents {
		presents[key] += value
	}
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
