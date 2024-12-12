package main

import (
	"fmt"

	"github.com/believer/aoc-2024/utils/files"
)

// BFS to find plant regions and perimeters
// For part 2 find corners of areas which show how many sides it has
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

type Location struct {
	r, c int
}

var directions = [][]int{
	{-1, 0}, // Above
	{0, 1},  // Right
	{1, 0},  // Below
	{0, -1}, // Left
}

func part1(name string) int {
	plants := files.ReadLines(name)
	grid := map[Location]byte{}
	visited := map[Location]bool{}

	rows := len(plants)
	cols := len(plants[0])
	price := 0

	for r := range rows {
		for c := range cols {
			grid[Location{r, c}] = plants[r][c]
		}
	}

	for r := range rows {
		for c := range cols {
			area, perimeter := findPlantBed(grid, visited, Location{r, c})
			price += len(area) * perimeter
		}
	}

	return price
}

func part2(name string) int {
	plants := files.ReadLines(name)
	grid := map[Location]byte{}
	visited := map[Location]bool{}

	rows := len(plants)
	cols := len(plants[0])
	price := 0

	for r := range rows {
		for c := range cols {
			grid[Location{r, c}] = plants[r][c]
		}
	}

	for r := range rows {
		for c := range cols {
			area, _ := findPlantBed(grid, visited, Location{r, c})
			price += len(area) * findCorners(area)
		}
	}

	return price
}

// The number of corners shows how many sides we have
func findCorners(area map[Location]bool) int {
	corners := 0

	cornerChecks := []struct {
		offsets        []Location
		requiredStates []bool
	}{
		// Outer corners
		{[]Location{{0, -1}, {-1, 0}}, []bool{false, false}}, // Top left
		{[]Location{{0, 1}, {-1, 0}}, []bool{false, false}},  // Top right
		{[]Location{{0, -1}, {1, 0}}, []bool{false, false}},  // Bottom left
		{[]Location{{0, 1}, {1, 0}}, []bool{false, false}},   // Bottom right

		// Inner corners
		{[]Location{{-1, -1}, {-1, 0}, {0, -1}}, []bool{false, true, true}}, // Inside top left
		{[]Location{{-1, 1}, {-1, 0}, {0, 1}}, []bool{false, true, true}},   // Inside top right
		{[]Location{{1, -1}, {1, 0}, {0, -1}}, []bool{false, true, true}},   // Inside bottom left
		{[]Location{{1, 1}, {1, 0}, {0, 1}}, []bool{false, true, true}},     // Inside bottom right
	}

	for a := range area {
		for _, check := range cornerChecks {
			match := true

			for i, offset := range check.offsets {
				neighbor := Location{a.r + offset.r, a.c + offset.c}

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

func findPlantBed(grid map[Location]byte, visited map[Location]bool, start Location) (map[Location]bool, int) {
	queue := []Location{start}
	perimeter := 0
	area := map[Location]bool{}

	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]

		if _, ok := visited[current]; ok {
			continue
		}

		neighbors := getNeighbors(grid, current)
		perimeter += 4 - len(neighbors)
		area[current] = true
		visited[current] = true

		for _, neighbor := range neighbors {
			queue = append(queue, neighbor)
		}
	}

	return area, perimeter
}

func getNeighbors(grid map[Location]byte, current Location) []Location {
	neighbors := []Location{}

	for _, d := range directions {
		next := Location{current.r + d[0], current.c + d[1]}

		if value, ok := grid[next]; ok && value == grid[current] {
			neighbors = append(neighbors, next)
		}
	}

	return neighbors
}
