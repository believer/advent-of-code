package utils

import "strconv"

type Point struct {
	x int
	y int
}

// Ints
// -----------------------------------------------------------

func Sum(nums []int) int {
	total := 0

	for _, n := range nums {
		total += n
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
