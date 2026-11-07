package grid

import (
	"errors"
	"fmt"
)

/* Grid
/* ====================================================== */

type Grid struct {
	Data   []byte
	Height int
	Width  int
}

// Create a grid from lines of equal width. Panics if the
// lines have different widths.
func New(lines []string) Grid {
	if len(lines) == 0 {
		return Grid{Data: []byte{}}
	}

	height := len(lines)
	width := len(lines[0])
	data := make([]byte, 0, height*width)

	for r, line := range lines {
		if len(line) != width {
			panic(fmt.Sprintf("grid: line %d has width %d, expected %d", r, len(line), width))
		}

		data = append(data, line...)
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

// Get the byte value at a given point.
// Use [GetWrapped] if grid needs to wrap around.
func (g *Grid) Get(p Point) byte {
	return g.Data[g.Width*p.Y+p.X]
}

// Get the byte value at a given point when the
// grid wraps around
func (g *Grid) GetWrapped(p Point) byte {
	// Wrap coordinates to ensure they stay within bounds
	wrappedX := (p.X%g.Width + g.Width) % g.Width
	wrappedY := (p.Y%g.Height + g.Height) % g.Height

	// Get the value at the wrapped position
	return g.Data[g.Width*wrappedY+wrappedX]
}

// Reports whether a given point is defined in the grid.
// If it exists, it includes the value at that point.
func (g *Grid) TryGet(p Point) (byte, bool) {
	if g.InBounds(p) {
		return g.Get(p), true
	}

	return 0, false
}

// Reports whether a given point is within the grids bounds
func (g *Grid) InBounds(p Point) bool {
	return p.X >= 0 && p.X < g.Width && p.Y >= 0 && p.Y < g.Height
}

// Update the grid at a given point with a given value
// See [UpdateWrapped] for wrapped grid.
func (g *Grid) Update(p Point, b byte) {
	g.Data[g.Width*p.Y+p.X] = b
}

// Update the grid at a given point with a given value
// in a wrapped grid.
func (g *Grid) UpdateWrapped(p Point, b byte) {
	// Wrap coordinates to ensure they stay within bounds
	wrappedX := (p.X%g.Width + g.Width) % g.Width
	wrappedY := (p.Y%g.Height + g.Height) % g.Height

	// Update the grid at the wrapped position
	g.Data[g.Width*wrappedY+wrappedX] = b
}

// Find exactly _one_ point for the given value
func (g *Grid) MustFind(needle byte) Point {
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

func (g *Grid) Find(needle byte) (Point, error) {
	for i, v := range g.Data {
		if v == needle {
			return Point{
				X: i % g.Width,
				Y: i / g.Width,
			}, nil
		}
	}

	return Point{}, errors.New("Point not found")
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

// In-bounds points next to p in the given directions,
// for example [CARDINALS] or [ALL_DIRECTIONS].
func (g *Grid) Neighbors(p Point, dirs []Point) []Point {
	neighbors := make([]Point, 0, len(dirs))

	for _, d := range dirs {
		if next := p.Add(d); g.InBounds(next) {
			neighbors = append(neighbors, next)
		}
	}

	return neighbors
}

// A copy of row y
func (g *Grid) Row(y int) []byte {
	return append([]byte{}, g.Data[y*g.Width:(y+1)*g.Width]...)
}

// A copy of column x
func (g *Grid) Col(x int) []byte {
	col := make([]byte, g.Height)

	for y := range g.Height {
		col[y] = g.Data[y*g.Width+x]
	}

	return col
}

// A deep copy of the grid
func (g *Grid) Clone() Grid {
	return Grid{
		Data:   append([]byte{}, g.Data...),
		Height: g.Height,
		Width:  g.Width,
	}
}

// A new grid with rows and columns swapped
func (g *Grid) Transpose() Grid {
	data := make([]byte, 0, len(g.Data))

	for x := range g.Width {
		for y := range g.Height {
			data = append(data, g.Data[y*g.Width+x])
		}
	}

	return Grid{
		Data:   data,
		Height: g.Width,
		Width:  g.Height,
	}
}

type Quadrant string

const (
	TopLeft     Quadrant = "TopLeft"
	TopRight    Quadrant = "TopRight"
	BottomLeft  Quadrant = "BottomLeft"
	BottomRight Quadrant = "BottomRight"
)

func (g *Grid) GetQuadrant(q Quadrant) []byte {
	var startX, startY, endX, endY int

	halfWidth := g.Width / 2
	halfHeight := g.Height / 2

	switch q {
	case TopLeft:
		startX, startY, endX, endY = 0, 0, halfWidth, halfHeight
	case TopRight:
		startX, startY, endX, endY = halfWidth+1, 0, g.Width, halfHeight
	case BottomLeft:
		startX, startY, endX, endY = 0, halfHeight+1, halfWidth, g.Height
	case BottomRight:
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
		g.GetQuadrant(TopLeft),
		g.GetQuadrant(TopRight),
		g.GetQuadrant(BottomLeft),
		g.GetQuadrant(BottomRight),
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
