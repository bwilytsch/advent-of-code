package day1

import (
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPartOne(t *testing.T) {
	assert.Equal(t, partOne("1969"), 654)
	assert.Equal(t, partOne("100756"), 33583)

	fileInput, _ := os.ReadFile("./input.txt")

	println(partOne(string(fileInput)))
}

func TestPartTwo(t *testing.T) {
	assert.Equal(t, partTwo("1969"), 966)
	assert.Equal(t, partTwo("100756"), 50346)

	fileInput, _ := os.ReadFile("./input.txt")

	println(partOne(string(fileInput)))
}
