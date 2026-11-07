package utils

import (
	"cmp"
	"strconv"
)

// Constraints
// -----------------------------------------------------------

type Signed interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64
}

type Unsigned interface {
	~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64
}

type Integer interface {
	Signed | Unsigned
}

type Float interface {
	~float32 | ~float64
}

type Number interface {
	Integer | Float
}

// Numbers
// -----------------------------------------------------------

func Sum[T Number](nums []T) T {
	var total T

	for _, n := range nums {
		total += n
	}

	return total
}

func Prod[T Number](nums []T) T {
	var total T = 1

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

// Ints returns all integers found in a string. A dash directly after
// a digit is treated as a separator, so "3-4" gives [3 4] (ranges) while
// "3 -4" gives [3 -4].
func Ints(s string) []int {
	nums := []int{}

	for i := 0; i < len(s); i++ {
		c := s[i]
		start := i

		if c == '-' && i+1 < len(s) && isDigit(s[i+1]) && (i == 0 || !isDigit(s[i-1])) {
			i++
		} else if !isDigit(c) {
			continue
		}

		for i+1 < len(s) && isDigit(s[i+1]) {
			i++
		}

		nums = append(nums, MustIntFromString(s[start:i+1]))
	}

	return nums
}

func isDigit(b byte) bool {
	return b >= '0' && b <= '9'
}

// Math
// -----------------------------------------------------------

func Abs[T Signed | Float](n T) T {
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
// |x1 - x2| + |y1 - y2|
func Manhattan(x, y int) int {
	return Abs(x) + Abs(y)
}

func Divmod(numerator, denominator int) (quotient, remainder int) {
	quotient = numerator / denominator
	remainder = numerator % denominator
	return
}

// Modulo that is never negative (as opposed to Go's %),
// useful for wrapping around. Mod(-7, 5) == 3.
func Mod[T Integer](a, m T) T {
	r := a % m

	if r < 0 {
		r += m
	}

	return r
}

// Greatest common divisor. The result is never negative.
func GCD[T Signed](a, b T) T {
	for b != 0 {
		a, b = b, a%b
	}

	return Abs(a)
}

// Least common multiple of one or more numbers. No arguments gives 1.
func LCM[T Signed](nums ...T) T {
	var result T = 1

	for _, n := range nums {
		result = Abs(result / GCD(result, n) * n)
	}

	return result
}

func Clamp[T cmp.Ordered](v, lo, hi T) T {
	return min(max(v, lo), hi)
}

// Slices
// -----------------------------------------------------------

func SlidingWindow[T any](slice []T, size int) [][]T {
	result := make([][]T, 0)

	for i := 0; i <= len(slice)-size; i++ {
		result = append(result, slice[i:i+size])
	}

	return result
}

// Split a slice into chunks of at most size elements
func Chunk[T any](slice []T, size int) [][]T {
	if size < 1 {
		panic("chunk size must be at least 1")
	}

	result := make([][]T, 0, (len(slice)+size-1)/size)

	for i := 0; i < len(slice); i += size {
		result = append(result, slice[i:min(i+size, len(slice))])
	}

	return result
}

// Reverse returns a reversed copy of the slice
func Reverse[T any](slice []T) []T {
	result := make([]T, len(slice))

	for i, v := range slice {
		result[len(slice)-1-i] = v
	}

	return result
}

// Swap rows and columns of a rectangular slice of slices
func Transpose[T any](rows [][]T) [][]T {
	if len(rows) == 0 {
		return [][]T{}
	}

	result := make([][]T, len(rows[0]))

	for c := range result {
		result[c] = make([]T, len(rows))

		for r := range rows {
			result[c][r] = rows[r][c]
		}
	}

	return result
}
