package day2

import (
	"fmt"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPartOne(t *testing.T) {
	assert.Equal(t, 3500, partOne("1,9,10,3,2,3,11,0,99,30,40,50"))
	assert.Equal(t, 2, partOne("2,4,4,5,99,0"))
	assert.Equal(t, 30, partOne("1,1,1,4,99,5,6,0,99"))

	input, _ := os.ReadFile("./input.txt")

	println("Day 1: ", partOne(string(input)))
}

func TestPartTwo(t *testing.T) {
	input, _ := os.ReadFile("./input.txt")
	result, _ := partTwo(string(input), 19690720)

	fmt.Printf("Day 2: %v", result)
}
