package main

import (
	"fmt"
	"math"
	"slices"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

// Hardest problem of the year. I had an idea of solving part 2
// using Gaussian elimination, but I don't have the bandwidth to
// write the code for it :'D Found that others had used Z3, so
// I tried that in Go, but found it confusing to understand.
// Did it in JS with z3-solver instead, but haven't included the code.

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
}

type Machine struct {
	lights   []int
	buttons  [][]int
	joltages []int
}

func part1(name string) (totalIterations int) {
	lines := files.ReadLines(name)
	machines := createMachines(lines)

	for _, m := range machines {
		start := []int{}

		for i, l := range m.lights {
			if l == 1 {
				start = append(start, i)
			}
		}

		maskedStart := toBitmask(start)
		maskedButtons := []int{}
		end := 0

		for _, b := range m.buttons {
			maskedButtons = append(maskedButtons, toBitmask(b))
		}

		current := []int{maskedStart}
		iterations := 0

		for !slices.Contains(current, end) {
			next := []int{}

			for _, c := range current {
				for _, b := range maskedButtons {
					next = append(next, c^b)
				}
			}

			current = next
			iterations += 1
		}

		totalIterations += iterations
	}

	return
}

func toBitmask(values []int) (mask int) {
	for _, i := range values {
		mask += int(math.Pow(2, float64(i)))
	}

	return
}

func createMachines(lines []string) (machines []Machine) {
	for _, l := range lines {
		lights := strings.Split(l[1:], "] ")
		buttonSchema := strings.Split(lights[1], " {")
		buttonData := strings.Fields(buttonSchema[0])
		joltageData := buttonSchema[1]

		var lightSchema []int
		for l := range strings.SplitSeq(lights[0], "") {
			switch l {
			case ".":
				lightSchema = append(lightSchema, 0)
			case "#":
				lightSchema = append(lightSchema, 1)
			}
		}

		var buttons [][]int
		for _, b := range buttonData {
			// Remove parenthesis
			b = b[1:]
			b = b[:len(b)-1]

			var buttonsSchema []int

			for b := range strings.SplitSeq(b, ",") {
				button := utils.MustIntFromString(b)
				buttonsSchema = append(buttonsSchema, button)
			}

			buttons = append(buttons, buttonsSchema)
		}

		var joltages []int
		for j := range strings.SplitSeq(joltageData[:len(joltageData)-1], ",") {
			joltages = append(joltages, utils.MustIntFromString(j))
		}

		machines = append(machines, Machine{
			lights:   lightSchema,
			buttons:  buttons,
			joltages: joltages,
		})
	}

	return
}
