package grid

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

var sample = []string{
	"#####",
	"#S.E#",
	"#.#.#",
	"#####",
}

func TestNew(t *testing.T) {
	g := New(sample)

	assert.Equal(t, 5, g.Width)
	assert.Equal(t, 4, g.Height)
	assert.Len(t, g.Data, 20)
}

func TestFromSize(t *testing.T) {
	g := FromSize(3, 2)

	assert.Equal(t, []byte("......"), g.Data)
	assert.Equal(t, 3, g.Width)
	assert.Equal(t, 2, g.Height)
}

func TestGet(t *testing.T) {
	g := New(sample)

	assert.Equal(t, byte('S'), g.Get(Point{1, 1}))
	assert.Equal(t, byte('E'), g.Get(Point{3, 1}))
	assert.Equal(t, byte('#'), g.Get(Point{0, 0}))
}

func TestGetWrapped(t *testing.T) {
	g := New(sample)

	assert.Equal(t, byte('S'), g.GetWrapped(Point{1 + 5, 1}))
	assert.Equal(t, byte('S'), g.GetWrapped(Point{1, 1 + 4}))
	assert.Equal(t, byte('S'), g.GetWrapped(Point{1 - 5, 1 - 4}))
}

func TestTryGetAndInBounds(t *testing.T) {
	g := New(sample)

	v, ok := g.TryGet(Point{1, 1})
	assert.True(t, ok)
	assert.Equal(t, byte('S'), v)

	for _, p := range []Point{{-1, 0}, {0, -1}, {5, 0}, {0, 4}} {
		_, ok := g.TryGet(p)
		assert.False(t, ok, "%v", p)
		assert.False(t, g.InBounds(p), "%v", p)
	}
}

func TestUpdate(t *testing.T) {
	g := New(sample)

	g.Update(Point{2, 1}, 'X')
	assert.Equal(t, byte('X'), g.Get(Point{2, 1}))

	g.UpdateWrapped(Point{2 + 5, 1 + 4}, 'Y')
	assert.Equal(t, byte('Y'), g.Get(Point{2, 1}))
}

func TestFind(t *testing.T) {
	g := New(sample)

	p, err := g.Find('E')
	assert.NoError(t, err)
	assert.Equal(t, Point{3, 1}, p)

	_, err = g.Find('Z')
	assert.Error(t, err)

	assert.Equal(t, Point{1, 1}, g.MustFind('S'))
	assert.Panics(t, func() { g.MustFind('Z') })
}

func TestFindAll(t *testing.T) {
	g := New(sample)

	assert.Equal(t, []Point{{2, 1}, {1, 2}, {3, 2}}, g.FindAll('.'))
	assert.Empty(t, g.FindAll('Z'))
}

func TestGetQuadrant(t *testing.T) {
	// 5x3 grid, middle row and column are skipped
	g := New([]string{
		"ab.cd",
		".....",
		"ef.gh",
	})

	assert.Equal(t, []byte("ab"), g.GetQuadrant(TopLeft))
	assert.Equal(t, []byte("cd"), g.GetQuadrant(TopRight))
	assert.Equal(t, []byte("ef"), g.GetQuadrant(BottomLeft))
	assert.Equal(t, []byte("gh"), g.GetQuadrant(BottomRight))
	assert.Len(t, g.GetQuadrants(), 4)
	assert.Panics(t, func() { g.GetQuadrant("nope") })
}

func TestPoint(t *testing.T) {
	p := Point{1, 2}

	assert.Equal(t, Point{2, 2}, p.Add(RIGHT))
	assert.Equal(t, Point{1, 1}, p.Add(UP))
	assert.Equal(t, 7, p.ManhattanDistance(Point{4, 6}))
	assert.Len(t, CARDINALS, 4)
	assert.Len(t, ALL_DIRECTIONS, 8)
}

func TestPoint3Distance(t *testing.T) {
	a := Point3{0, 0, 0}

	assert.Equal(t, 14, a.Distance(Point3{1, 2, 3}))
	assert.Equal(t, 0, a.Distance(a))
}

func TestNewEmpty(t *testing.T) {
	g := New(nil)

	assert.Equal(t, 0, g.Width)
	assert.Equal(t, 0, g.Height)
	assert.Empty(t, g.Data)
}

func TestNewRaggedPanics(t *testing.T) {
	assert.PanicsWithValue(t, "grid: line 1 has width 2, expected 3", func() {
		New([]string{"abc", "de"})
	})
}

func TestPointValueReceivers(t *testing.T) {
	// Callable on non-addressable values
	assert.Equal(t, Point{1, 1}, Point{0, 0}.Add(Point{1, 1}))
	assert.Equal(t, Point{-1, 1}, Point{0, 2}.Sub(Point{1, 1}))
	assert.Equal(t, Point{3, -6}, Point{1, -2}.Scale(3))
	assert.Equal(t, 2, Point{0, 0}.ManhattanDistance(Point{1, 1}))
	assert.Equal(t, 14, Point3{}.Distance(Point3{1, 2, 3}))
}

func TestTurning(t *testing.T) {
	// Y grows downwards, so turning right from UP is RIGHT
	assert.Equal(t, RIGHT, UP.TurnRight())
	assert.Equal(t, DOWN, RIGHT.TurnRight())
	assert.Equal(t, LEFT, DOWN.TurnRight())
	assert.Equal(t, UP, LEFT.TurnRight())

	assert.Equal(t, LEFT, UP.TurnLeft())
	assert.Equal(t, UP, RIGHT.TurnLeft())

	assert.Equal(t, DOWN, UP.Opposite())
	assert.Equal(t, LEFT, RIGHT.Opposite())
	assert.Equal(t, BOTTOMRIGHT, TOPLEFT.Opposite())
}

func TestNeighbors(t *testing.T) {
	g := New([]string{
		"abc",
		"def",
		"ghi",
	})

	assert.ElementsMatch(t, []Point{{1, 0}, {0, 1}}, g.Neighbors(Point{0, 0}, CARDINALS))
	assert.Len(t, g.Neighbors(Point{1, 1}, CARDINALS), 4)
	assert.Len(t, g.Neighbors(Point{1, 1}, ALL_DIRECTIONS), 8)
	assert.Len(t, g.Neighbors(Point{0, 0}, ALL_DIRECTIONS), 3)
}

func TestRowColClone(t *testing.T) {
	g := New([]string{
		"abc",
		"def",
	})

	assert.Equal(t, []byte("def"), g.Row(1))
	assert.Equal(t, []byte("be"), g.Col(1))

	c := g.Clone()
	c.Update(Point{0, 0}, 'X')
	assert.Equal(t, byte('a'), g.Get(Point{0, 0}), "clone is independent")
	assert.Equal(t, byte('X'), c.Get(Point{0, 0}))
}

func TestGridTranspose(t *testing.T) {
	g := New([]string{
		"abc",
		"def",
	})

	tr := g.Transpose()

	assert.Equal(t, 2, tr.Width)
	assert.Equal(t, 3, tr.Height)
	assert.Equal(t, []byte("adbecf"), tr.Data)
}
