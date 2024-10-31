package day1

import (
	"math"
	"strconv"
	"strings"
)

func getFuelFromMass(mass float64) float64 {
	value := math.Floor(mass/3.0) - 2

	if value < 0.0 {
		return 0.0
	}

	return value
}

func partOne(input string) int {
	sum := 0.0

	for _, line := range strings.Split(strings.TrimSuffix(input, "\n"), "\n") {
		mass, _ := strconv.ParseFloat(line, 10)

		sum += getFuelFromMass(mass)
	}

	return int(sum)
}

func calcTotalFuel(mass float64, cache map[float64]float64) float64 {
	cachedValue, ok := cache[mass]

	if ok {
		return cachedValue
	}

	if mass <= 0.0 {
		return 0.0
	}

	fuel := getFuelFromMass(mass)

	cache[mass] = fuel

	return fuel + calcTotalFuel(fuel, cache)
}

func partTwo(input string) int {
	sum := 0.0
	cache := make(map[float64]float64)

	for _, line := range strings.Split(strings.TrimSuffix(input, "\n"), "\n") {
		mass, _ := strconv.ParseFloat(line, 10)

		sum += calcTotalFuel(mass, cache)
	}

	return int(sum)
}
