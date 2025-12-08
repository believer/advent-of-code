package grid

import "github.com/believer/aoc-utils/utils"

/*
	2D Point

/* ======================================================
*/
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

/* 3D Point
/* ====================================================== */

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
