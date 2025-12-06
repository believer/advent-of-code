package utils

import "strconv"

// Ints
// -----------------------------------------------------------

func Sum(nums []int) int {
	total := 0

	for _, n := range nums {
		total += n
	}

	return total
}

func Prod(nums []int) int {
	total := 1

	for _, n := range nums {
		total *= n
	}

	return total
}

func MustIntFromString(s string) int {
	v, err := strconv.Atoi(s)

	if err != nil {
		panic(err)
	}

	return v
}

// Math
// -----------------------------------------------------------

// For float64, use math.Abs
func Abs(n int) int {
	if n < 0 {
		return -n
	}

	return n
}

// Manhattan distance
// The sum of the absolute values of two points
// This function can be used when the starting point can be considered
// the origin (0,0). The original formula is
//
// |x1 - x2| + \y1 - y2|
func Manhattan(x, y int) int {
	return Abs(x) + Abs(y)
}

func Divmod(numerator, denominator int) (quotient, remainder int) {
	quotient = numerator / denominator
	remainder = numerator % denominator
	return
}

// Slices
// -----------------------------------------------------------

func SlidingWindow(slice []int, size int) [][]int {
	var result [][]int = make([][]int, 0)
	for i := 0; i <= len(slice)-size; i++ {
		window := slice[i : i+size]
		result = append(result, window)
	}
	return result
}
