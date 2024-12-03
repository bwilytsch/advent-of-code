package day_1

import (
	"fmt"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPartOne(t *testing.T) {
	example := `3   4
		4   3
		2   5
		1   3
		3   9
		3   3`

	assert.Equal(t, 11, partOne(example))

	// Real input
	input, error := os.ReadFile("./day_1_input.txt")

	if error == nil {
		result := partOne(string(input))
		fmt.Println("Day 1 | Part 1 = ", result)
	}
}

// func BenchmarkPartOne(b *testing.B) {
// 	fileInput, _ := os.ReadFile("./input_day_1.txt")
//
// 	for i := 0; i < b.N; i++ {
// 		partOne(string(fileInput))
// 	}
// }

func TestPartTwo(t *testing.T) {
	example := `3   4
		4   3
		2   5
		1   3
		3   9
		3   3`

	assert.Equal(t, 31, partTwo(example))

	// Real input
	input, error := os.ReadFile("./day_1_input.txt")
	if error == nil {
		result := partTwo(string(input))
		fmt.Println("Day 1 | Part 2 = ", result)
	}
}

// func BenchmarkPartTwo(b *testing.B) {
// 	fileInput, _ := os.ReadFile("./input_day_1.txt")
//
// 	for i := 0; i < b.N; i++ {
// 		partTwo(string(fileInput))
// 	}
// }
