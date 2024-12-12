package grid

/* Point
/* ====================================================== */

type Point struct {
	X, Y int
}

var UP = Point{0, -1}
var DOWN = Point{0, 1}
var RIGHT = Point{1, 0}
var LEFT = Point{-1, 0}
var CARDINALS = []Point{UP, DOWN, LEFT, RIGHT}

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
