package main

import (
	"fmt"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/grid"
)

// BFS to find plant regions and perimeters
// For part 2 find corners of areas which show how many sides it has
// Added a similar Grid/Point solution like I've used in my Rust solutions
// in previous years. Made things faster (one dimensional array of bytes) and cleaner!
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	lines := files.ReadLines(name)
	plants := grid.New(lines)
	visited := map[grid.Point]bool{}
	price := 0

	for y := range plants.Height {
		for x := range plants.Width {
			area, perimeter := findPlantBed(plants, visited, grid.Point{X: x, Y: y})
			price += len(area) * perimeter
		}
	}

	return price
}

func part2(name string) int {
	lines := files.ReadLines(name)
	plants := grid.New(lines)
	visited := map[grid.Point]bool{}
	price := 0

	for y := range plants.Height {
		for x := range plants.Width {
			area, _ := findPlantBed(plants, visited, grid.Point{X: x, Y: y})
			price += len(area) * findCorners(area)
		}
	}

	return price
}

// The number of corners shows how many sides we have
func findCorners(area map[grid.Point]bool) int {
	corners := 0

	cornerChecks := []struct {
		offsets        []grid.Point
		requiredStates []bool
	}{
		// Outer corners
		{[]grid.Point{{Y: 0, X: -1}, {Y: -1, X: 0}}, []bool{false, false}}, // Top left
		{[]grid.Point{{Y: 0, X: 1}, {Y: -1, X: 0}}, []bool{false, false}},  // Top right
		{[]grid.Point{{Y: 0, X: -1}, {Y: 1, X: 0}}, []bool{false, false}},  // Bottom left
		{[]grid.Point{{Y: 0, X: 1}, {Y: 1, X: 0}}, []bool{false, false}},   // Bottom right

		// Inner corners
		{[]grid.Point{{Y: -1, X: -1}, {Y: -1, X: 0}, {Y: 0, X: -1}}, []bool{false, true, true}}, // Inside top left
		{[]grid.Point{{Y: -1, X: 1}, {Y: -1, X: 0}, {Y: 0, X: 1}}, []bool{false, true, true}},   // Inside top right
		{[]grid.Point{{Y: 1, X: -1}, {Y: 1, X: 0}, {Y: 0, X: -1}}, []bool{false, true, true}},   // Inside bottom left
		{[]grid.Point{{Y: 1, X: 1}, {Y: 1, X: 0}, {Y: 0, X: 1}}, []bool{false, true, true}},     // Inside bottom right
	}

	for a := range area {
		for _, check := range cornerChecks {
			match := true

			for i, offset := range check.offsets {
				neighbor := a.Add(offset)

				if _, ok := area[neighbor]; ok != check.requiredStates[i] {
					match = false
					break
				}
			}

			if match {
				corners++
			}
		}
	}

	return corners
}

func findPlantBed(g grid.Grid, visited map[grid.Point]bool, start grid.Point) (map[grid.Point]bool, int) {
	queue := []grid.Point{start}
	area := map[grid.Point]bool{}
	perimeter := 0

	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]

		if _, ok := visited[current]; ok {
			continue
		}

		neighbors := getNeighbors(g, current)
		perimeter += 4 - len(neighbors)
		area[current] = true
		visited[current] = true

		for _, neighbor := range neighbors {
			queue = append(queue, neighbor)
		}
	}

	return area, perimeter
}

func getNeighbors(g grid.Grid, current grid.Point) []grid.Point {
	neighbors := []grid.Point{}

	for _, d := range grid.CARDINALS {
		next := current.Add(d)

		if value, ok := g.TryGet(next); ok && value == g.Get(current) {
			neighbors = append(neighbors, next)
		}
	}

	return neighbors
}
