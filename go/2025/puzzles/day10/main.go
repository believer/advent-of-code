package main

import (
	"fmt"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

type Machine struct {
	lights  []int
	buttons [][]int
	joltage []int
}

func part1(name string) int {
	lines := files.ReadLines(name)
	var machines []Machine

	for _, l := range lines {
		lights := strings.Split(l[1:], "] ")
		buttonSchema := strings.Split(lights[1], " {")
		buttonSchema = strings.Fields(buttonSchema[0])

		lightSchema := make([]int, len(lights[0]))
		var buttons [][]int

		for _, b := range buttonSchema {
			// Remove parenthesis
			b = b[1:]
			b = b[:len(b)-1]

			// Create a matrix for the button presses
			buttonsSchema := make([]int, len(lightSchema))

			for b := range strings.SplitSeq(b, ",") {
				button := utils.MustIntFromString(b)
				buttonsSchema[button] = 1
			}

			buttons = append(buttons, buttonsSchema)
		}

		machines = append(machines, Machine{
			lights:  lightSchema,
			buttons: buttons,
		})
	}

	for _, m := range machines[:1] {
		solveMachine(m)
	}

	return 0
}

func part2(name string) int {
	return 0
}

// Gauss elimination
func solveMachine(m Machine) int {
	N := len(m.lights)
	M := len(m.buttons)

	// Create the augmented matrix [A|B]
	augmentedMatrix := make([][]int, N)
	for i := range N {
		augmentedMatrix[i] = make([]int, M+1)

		// Copy A (buttons)
		for j := range M {
			augmentedMatrix[i][j] = m.buttons[j][i]
		}

		// Copy B (lights)
		augmentedMatrix[i][M] = m.lights[i]
	}

	fmt.Println(augmentedMatrix)

	return 0
}
