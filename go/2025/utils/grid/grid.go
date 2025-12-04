package grid

import (
	"fmt"

	"github.com/believer/aoc-2025/utils"
)

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

func (p *Point) ManhattanDistance(p2 Point) int {
	return utils.Abs(p2.X-p.X) + utils.Abs(p2.Y-p.Y)
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

func FromSize(width, height int) Grid {
	data := make([]byte, width*height)

	for i := range data {
		data[i] = '.'
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

func (g *Grid) GetWrapped(p Point) byte {
	// Wrap coordinates to ensure they stay within bounds
	wrappedX := (p.X%g.Width + g.Width) % g.Width
	wrappedY := (p.Y%g.Height + g.Height) % g.Height

	// Update the grid at the wrapped position
	return g.Data[g.Width*wrappedY+wrappedX]
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

func (g *Grid) UpdateWrapped(p Point, b byte) {
	// Wrap coordinates to ensure they stay within bounds
	wrappedX := (p.X%g.Width + g.Width) % g.Width
	wrappedY := (p.Y%g.Height + g.Height) % g.Height

	// Update the grid at the wrapped position
	g.Data[g.Width*wrappedY+wrappedX] = b
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

func (g *Grid) GetQuadrant(q int) []byte {
	var startX, startY, endX, endY int

	halfWidth := g.Width / 2
	halfHeight := g.Height / 2

	switch q {
	case 1: // Top-Left
		startX, startY, endX, endY = 0, 0, halfWidth, halfHeight
	case 2: // Top-Right
		startX, startY, endX, endY = halfWidth+1, 0, g.Width, halfHeight
	case 3: // Bottom-Left
		startX, startY, endX, endY = 0, halfHeight+1, halfWidth, g.Height
	case 4: // Bottom-Right
		startX, startY, endX, endY = halfWidth+1, halfHeight+1, g.Width, g.Height
	default:
		panic("Invalid quadrant number")
	}

	quadrantData := make([]byte, 0, (endX-startX)*(endY-startY))
	for y := startY; y < endY; y++ {
		quadrantData = append(quadrantData, g.Data[y*g.Width+startX:y*g.Width+endX]...)
	}

	return quadrantData
}

func (g *Grid) GetQuadrants() [][]byte {
	return [][]byte{
		g.GetQuadrant(1),
		g.GetQuadrant(2),
		g.GetQuadrant(3),
		g.GetQuadrant(4),
	}
}

func (g *Grid) Debug() {
	for y := range g.Height {
		for x := range g.Width {
			fmt.Print(string(g.Get(Point{X: x, Y: y})))
		}
		fmt.Println()
	}

	fmt.Println()
}
