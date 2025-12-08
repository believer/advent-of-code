package main

import (
	"fmt"
	"sort"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt", 1000))
	fmt.Println("Part 2: ", part2("input.txt"))
}

type Point3 struct {
	X, Y, Z int
}

func (p *Point3) Distance(q Point3) int {
	dx := p.X - q.X
	dy := p.Y - q.Y
	dz := p.Z - q.Z

	// Use only squared distances to avoid converting
	// to floats for math.X calculations
	return dx*dx + dy*dy + dz*dz
}

type Edge struct {
	P1, P2   int
	Distance int
}

type Edges []Edge

// Sort interface
func (e Edges) Len() int           { return len(e) }
func (e Edges) Swap(i, j int)      { e[i], e[j] = e[j], e[i] }
func (e Edges) Less(i, j int) bool { return e[i].Distance < e[j].Distance }

// Disjoint Set Union (DSU)
type DSU struct {
	parent []int
	size   []int
	count  int
}

func NewDSU(n int) *DSU {
	dsu := &DSU{
		parent: make([]int, n),
		size:   make([]int, n),
		count:  n,
	}

	for i := range n {
		dsu.parent[i] = i // Each element is its own parent
		dsu.size[i] = 1   // Each set starts with size 1
	}

	return dsu
}

func (d *DSU) Find(i int) int {
	if d.parent[i] == i {
		return i
	}

	// Path compression: set parent[i] directly to the root
	d.parent[i] = d.Find(d.parent[i])

	return d.parent[i]
}

// Merge sets that contain i and j. Report whether a successful
// merge occurred (when i and j were in different sets)
func (d *DSU) Union(i, j int) bool {
	a := d.Find(i)
	b := d.Find(j)

	if a == b {
		return false
	}

	// Swap to make sure that a is the larger set's root
	if d.size[a] < d.size[b] {
		a, b = b, a
	}

	d.parent[b] = a
	d.size[a] += d.size[b]
	d.count -= 1

	return true
}

func part1(name string, count int) int {
	lines := files.ReadLines(name)

	// Create all coordinates from input
	coords := []Point3{}

	for _, l := range lines {
		c := strings.Split(l, ",")

		coord := Point3{
			X: utils.MustIntFromString(c[0]),
			Y: utils.MustIntFromString(c[1]),
			Z: utils.MustIntFromString(c[2]),
		}

		coords = append(coords, coord)
	}

	// Find all potential edges
	edges := []Edge{}
	n := len(coords)

	for i := range n {
		for j := i + 1; j < n; j++ {
			if i == j {
				continue
			}

			distance := coords[i].Distance(coords[j])

			edge := Edge{P1: i, P2: j, Distance: distance}
			edges = append(edges, edge)
		}
	}

	// Sort by distance
	sort.Sort(Edges(edges))

	dsu := NewDSU(n)

	for i := range count {
		dsu.Union(edges[i].P1, edges[i].P2)
	}

	var sizes []int

	for i := range len(lines) {
		if dsu.parent[i] == i {
			sizes = append(sizes, dsu.size[i])
		}
	}

	sort.Slice(sizes, func(a, b int) bool {
		return sizes[a] > sizes[b]
	})

	return sizes[0] * sizes[1] * sizes[2]
}

func part2(name string) int {
	lines := files.ReadLines(name)

	// Create all coordinates from input
	coords := []Point3{}

	for _, l := range lines {
		c := strings.Split(l, ",")

		coord := Point3{
			X: utils.MustIntFromString(c[0]),
			Y: utils.MustIntFromString(c[1]),
			Z: utils.MustIntFromString(c[2]),
		}

		coords = append(coords, coord)
	}

	// Find all potential edges
	edges := []Edge{}
	n := len(coords)

	for i := range n {
		for j := i + 1; j < n; j++ {
			if i == j {
				continue
			}

			distance := coords[i].Distance(coords[j])

			edge := Edge{P1: i, P2: j, Distance: distance}
			edges = append(edges, edge)
		}
	}

	// Sort by distance
	sort.Sort(Edges(edges))

	dsu := NewDSU(n)
	cable := 0

	for _, edge := range edges {
		x1 := coords[edge.P1].X
		x2 := coords[edge.P2].X

		cable = x1 * x2

		dsu.Union(edge.P1, edge.P2)

		if dsu.count == 1 {
			break
		}
	}

	return cable
}
