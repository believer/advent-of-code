package main

import (
	"fmt"
	"regexp"

	"github.com/believer/aoc-2024/utils"
	"github.com/believer/aoc-2024/utils/files"
)

// Needed some tips to get the solution for part 2 and got to
// revive my linear algebra knowledge from 10+ years ago. I really
// enjoyed matrix math back in the day so that was fun :D
//
// Had made an error in the determinant Ax/Ay zero check and used && instead of ||
// It worked for test input, but not real. Took me way too long to find...
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
//
// If we restructure this, we need to solve two equations:
//
// Ax * a + Bx * b = PrizeX
// Ay * a + By * b = PrizeY
//
// Where a and b represent button presses. 0 <= a <= 100
func part1(name string) int {
	prizes := files.ReadParagraphs(name)
	tokens := 0

	for _, prize := range prizes {
		Ax, Ay, Bx, By, prizeX, prizeY := getEquationValues(prize)

		for a := range 101 {
			for b := range 101 {
				buttonA := Ax*a + Bx*b
				buttonB := Ay*a + By*b

				if buttonA == prizeX && buttonB == prizeY {
					// A tokens cost three times more than B tokens
					tokens += a*3 + b
				}
			}
		}
	}

	return tokens
}

func part2(name string) int {
	return solve(name, 10000000000000)
}

func getEquationValues(prize []string) (int, int, int, int, int, int) {
	re := regexp.MustCompile(`\d+`)
	Ax, Ay, Bx, By := 0, 0, 0, 0
	prizeX, prizeY := 0, 0

	// Extract values from instructions
	for i, part := range prize {
		matches := re.FindAllString(part, -1)
		x, y := utils.MustIntFromString(matches[0]), utils.MustIntFromString(matches[1])

		switch i {
		case 0:
			Ax, Ay = x, y
		case 1:
			Bx, By = x, y
		case 2:
			prizeX, prizeY = x, y
		}
	}

	return Ax, Ay, Bx, By, prizeX, prizeY
}

// We can use linear algebra to solve this.
//
// ax + by = c1
// cx + dy = c2
//
// Which in matrix form looks like:
//
// [a1 b1] [x]   [c1]
// [a2 b2] [y] = [c2]
//
// Using the first matrix (the coefficient matrix), A, we can
// calculate the determinant:
//
// det(A) = a1b2 - a2b1
//
// This is only solvable if the determinant is _not_ zero
// From this we can use Cramer's rule to calculate x and y.
//
// [c1 b1]
// [c2 b2] = Ax
//
// [a1 c1]
// [a2 c2] = Ay
//
// Calculate the determinants like for A and then divide by det(A)
// to get the solutions for x and y:
//
// x = det(Ax) / det(A)
// y = det(Ay) / det(A)
func solve(name string, offset int) int {
	prizes := files.ReadParagraphs(name)
	tokens := 0

	for _, prize := range prizes {
		a1, a2, b1, b2, c1, c2 := getEquationValues(prize)

		// Add offset
		c1 += offset
		c2 += offset

		// Calculate determinant of coefficient matrix
		detA := a1*b2 - a2*b1

		// If the determinant is zero, there are either no or infinite solutions
		if detA == 0 {
			continue
		}

		detAx := (c1*b2 - c2*b1)
		detAy := (c2*a1 - c1*a2)

		// Ensure that we have integer solutions
		if detAx%detA != 0 || detAy%detA != 0 {
			continue
		}

		x := detAx / detA
		y := detAy / detA

		// Only positive solutions
		if x < 0 || y < 0 {
			continue
		}

		tokens += 3*x + y
	}

	return tokens
}
