package grid

import "fmt"

/* Point
/* ====================================================== */

type Point struct {
	X, Y int
}

var UP = Point{0, -1}
var DOWN = Point{0, 1}
var RIGHT = Point{1, 0}
var LEFT = Point{-1, 0}
var TOPLEFT = Point{-1, -1}
var TOPRIGHT = Point{1, -1}
var BOTTOMLEFT = Point{-1, 1}
var BOTTOMRIGHT = Point{1, 1}

var CARDINALS = []Point{UP, DOWN, LEFT, RIGHT}
var ALL_DIRECTIONS = []Point{
	TOPLEFT,
	UP,
	TOPRIGHT,
	RIGHT,
	BOTTOMRIGHT,
	DOWN,
	BOTTOMLEFT,
	LEFT,
}

func (p *Point) Add(p2 Point) Point {
	return Point{p.X + p2.X, p.Y + p2.Y}
}

/* Grid
/* ====================================================== */

type Grid struct {
	Data   []byte
	Height int
	Width  int
}

func New(lines []string) Grid {
	data := []byte{}
	height := len(lines)
	width := len(lines[0])

	for r := range height {
		for c := range width {
			current := lines[r][c]
			data = append(data, current)
		}
	}

	return Grid{
		Data:   data,
		Height: height,
		Width:  width,
	}
}

func (g *Grid) Get(p Point) byte {
	return g.Data[g.Width*p.Y+p.X]
}

func (g *Grid) Contains(p Point) (byte, bool) {
	if p.X >= 0 && p.X < g.Width && p.Y >= 0 && p.Y < g.Height {
		return g.Get(p), true
	}

	return 0, false
}

func (g *Grid) Update(p Point, b byte) {
	g.Data[g.Width*p.Y+p.X] = b
}

// Find exactly _one_ point for the given value
func (g *Grid) Find(needle byte) Point {
	for i, v := range g.Data {
		if v == needle {
			return Point{
				X: i % g.Width,
				Y: i / g.Width,
			}
		}
	}

	panic("Point not found")
}

// Find _all_ points for the given value
func (g *Grid) FindAll(needle byte) []Point {
	points := []Point{}

	for i, v := range g.Data {
		if v == needle {
			points = append(points, Point{
				X: i % g.Width,
				Y: i / g.Width,
			})
		}
	}

	return points
}

func (g *Grid) Debug() {
	for y := range g.Height {
		for x := range g.Width {
			fmt.Print(string(g.Get(Point{X: x, Y: y})))
		}
		fmt.Println()
	}
}
