package main

import (
	"fmt"
	"sort"
	"strings"

	"github.com/believer/aoc-utils/data/dsu"
	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/graph"
	"github.com/believer/aoc-utils/grid"
	"github.com/believer/aoc-utils/utils"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt", 1000))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string, count int) int {
	// Create all coordinates from input
	coords := getCoords(name)

	// Find all potential edges
	edges := getEdges(coords)

	dsu := dsu.New(len(coords))

	// Create X unions in the DSU
	for i := range count {
		dsu.Union(edges[i].P1, edges[i].P2)
	}

	// Get sizes for each set and sort them descending
	var sizes []int

	for i := range len(coords) {
		if dsu.Parent[i] == i {
			sizes = append(sizes, dsu.Size[i])
		}
	}

	sort.Slice(sizes, func(a, b int) bool {
		return sizes[a] > sizes[b]
	})

	// Multiply three largest sets
	return sizes[0] * sizes[1] * sizes[2]
}

func part2(name string) (cable int) {
	coords := getCoords(name)
	edges := getEdges(coords)
	dsu := dsu.New(len(coords))

	// Check each edge until we have a single circuit
	// The answer is the multiplication of the X-values
	// for those nodes
	for _, edge := range edges {
		x1 := coords[edge.P1].X
		x2 := coords[edge.P2].X

		cable = x1 * x2

		dsu.Union(edge.P1, edge.P2)

		if dsu.Count == 1 {
			break
		}
	}

	return
}

func getCoords(name string) (coords []grid.Point3) {
	lines := files.ReadLines(name)

	for _, l := range lines {
		c := strings.Split(l, ",")

		coord := grid.Point3{
			X: utils.MustIntFromString(c[0]),
			Y: utils.MustIntFromString(c[1]),
			Z: utils.MustIntFromString(c[2]),
		}

		coords = append(coords, coord)
	}

	return
}

func getEdges(coords []grid.Point3) (edges graph.Edges) {
	n := len(coords)

	for i := range n {
		for j := i + 1; j < n; j++ {
			if i == j {
				continue
			}

			distance := coords[i].Distance(coords[j])

			edge := graph.Edge{P1: i, P2: j, Distance: distance}
			edges = append(edges, edge)
		}
	}

	// Sort by distance
	sort.Sort(graph.Edges(edges))

	return
}
