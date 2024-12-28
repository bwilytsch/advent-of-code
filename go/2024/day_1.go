package day_1

import (
	"math"
	"sort"
	"strconv"
	"strings"
)

func partOne(input string) int {
	lines := strings.Split(strings.TrimSpace(input), "\n")

	var sum float64 = 0

	left := []int{}
	right := []int{}

	for _, line := range lines {
		ele := strings.Fields(strings.TrimSpace(line))

		l, _ := strconv.Atoi(ele[0])
		r, _ := strconv.Atoi(ele[1])

		left = append(left, l)
		right = append(right, r)
	}

	sort.Ints(left)
	sort.Ints(right)

	for i, lValue := range left {
		rValue := right[i]

		sum += math.Abs(float64(lValue) - float64(rValue))
	}

	return int(sum)
}

func partTwo(input string) int {
	lot := make(map[int]int)
	lines := strings.Split(strings.TrimSpace(input), "\n")

	left := []int{}

	for _, line := range lines {
		ele := strings.Fields(strings.TrimSpace(line))

		l, _ := strconv.Atoi(ele[0])
		r, _ := strconv.Atoi(ele[1])

		_, exists := lot[r]

		if exists {
			lot[r] += 1
		} else {
			lot[r] = 1
		}

		left = append(left, l)
	}

	sum := 0

	for _, value := range left {
		count, exists := lot[value]

		if exists {
			sum += value * count
		}
	}

	return int(sum)
}
