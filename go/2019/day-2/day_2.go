package day2

import (
	"errors"
	"strconv"
	"strings"
)

func calc(nums []int) int {
	total := len(nums)
	pointer := 0

	for pointer < total {
		op := nums[pointer]

		switch op {
		case 1:
			// Add
			a, b, target := nums[pointer+1],
				nums[pointer+2],
				nums[pointer+3]

			nums[target] = nums[a] + nums[b]

			pointer += 4
		case 2:
			// Mult
			a, b, target := nums[pointer+1],
				nums[pointer+2],
				nums[pointer+3]

			nums[target] = nums[a] * nums[b]

			pointer += 4
		default:
			pointer = total
		}
	}

	return nums[0]
}

func partOne(seq string) int {
	arr := strings.Split(seq, ",")
	total := len(arr)
	nums := make([]int, total)

	for i, s := range arr {
		v, _ := strconv.Atoi(s)
		nums[i] = v
	}

	if len(nums) > 12 {
		nums[1] = 12
		nums[2] = 2
	}

	return calc(nums)

}

func partTwo(input string, expected int) ([2]int, error) {
	arr := strings.Split(input, ",")
	total := len(arr)
	nums := make([]int, total)

	for i, s := range arr {
		v, _ := strconv.Atoi(s)
		nums[i] = v
	}

	for noun := range 100 {
		for verb := range 100 {
			clone := make([]int, len(nums))
			copy(clone, nums)

			clone[1] = noun
			clone[2] = verb

			if expected == calc(clone) {
				return [2]int{noun, verb}, nil
			}
		}
	}

	return [2]int{-1, -1}, errors.New("division by zero")
}
