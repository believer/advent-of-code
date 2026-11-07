package utils

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSum(t *testing.T) {
	t.Run("Summarizes numbers", func(t *testing.T) {
		assert.Equal(t, 20, Sum([]int{12, 1, 7}))
	})

	t.Run("Works with other numeric types", func(t *testing.T) {
		assert.Equal(t, int64(6), Sum([]int64{1, 2, 3}))
		assert.InDelta(t, 3.5, Sum([]float64{1.5, 2}), 1e-9)
	})

	t.Run("Empty is zero", func(t *testing.T) {
		assert.Equal(t, 0, Sum([]int{}))
	})
}

func TestProd(t *testing.T) {
	assert.Equal(t, 24, Prod([]int{2, 3, 4}))
	assert.Equal(t, uint64(6), Prod([]uint64{1, 2, 3}))
	assert.Equal(t, 1, Prod([]int{}))
}

func TestMustIntFromString(t *testing.T) {
	assert.Equal(t, -12, MustIntFromString("-12"))
	assert.Panics(t, func() { MustIntFromString("x") })
}

func TestAbs(t *testing.T) {
	assert.Equal(t, 3, Abs(-3))
	assert.Equal(t, 3, Abs(3))
	assert.Equal(t, int64(3), Abs(int64(-3)))
	assert.InDelta(t, 2.5, Abs(-2.5), 1e-9)
}

func TestManhattan(t *testing.T) {
	assert.Equal(t, 7, Manhattan(-3, 4))
}

func TestDivmod(t *testing.T) {
	q, r := Divmod(17, 5)

	assert.Equal(t, 3, q)
	assert.Equal(t, 2, r)
}

func TestMod(t *testing.T) {
	assert.Equal(t, 2, Mod(7, 5))
	assert.Equal(t, 3, Mod(-7, 5))
	assert.Equal(t, 0, Mod(-5, 5))
	assert.Equal(t, int64(4), Mod(int64(-1), int64(5)))
}

func TestGCDAndLCM(t *testing.T) {
	assert.Equal(t, 6, GCD(12, 18))
	assert.Equal(t, 5, GCD(0, 5))
	assert.Equal(t, 6, GCD(-12, 18))
	assert.Equal(t, 36, LCM(12, 18))
	assert.Equal(t, 60, LCM(3, 4, 5))
	assert.Equal(t, 7, LCM(7))
	assert.Equal(t, 1, LCM[int]())
}

func TestClamp(t *testing.T) {
	assert.Equal(t, 5, Clamp(5, 0, 10))
	assert.Equal(t, 0, Clamp(-5, 0, 10))
	assert.Equal(t, 10, Clamp(50, 0, 10))
	assert.Equal(t, "b", Clamp("z", "a", "b"))
}

func TestSlidingWindow(t *testing.T) {
	assert.Equal(t, [][]int{{1, 2}, {2, 3}, {3, 4}}, SlidingWindow([]int{1, 2, 3, 4}, 2))
	assert.Equal(t, [][]string{{"a", "b"}}, SlidingWindow([]string{"a", "b"}, 2))
	assert.Empty(t, SlidingWindow([]int{1}, 2))
}

func TestChunk(t *testing.T) {
	assert.Equal(t, [][]int{{1, 2}, {3, 4}, {5}}, Chunk([]int{1, 2, 3, 4, 5}, 2))
	assert.Empty(t, Chunk([]int{}, 2))
	assert.Panics(t, func() { Chunk([]int{1}, 0) })
}

func TestReverse(t *testing.T) {
	original := []int{1, 2, 3}

	assert.Equal(t, []int{3, 2, 1}, Reverse(original))
	assert.Equal(t, []int{1, 2, 3}, original, "does not modify input")
}

func TestTranspose(t *testing.T) {
	assert.Equal(
		t,
		[][]int{{1, 4}, {2, 5}, {3, 6}},
		Transpose([][]int{{1, 2, 3}, {4, 5, 6}}),
	)
	assert.Empty(t, Transpose([][]int{}))
}

func TestInts(t *testing.T) {
	assert.Equal(t, []int{-1, 2, 30}, Ints("x=-1, y 2;30"))
	assert.Equal(t, []int{3, 4}, Ints("3-4"), "dash after a digit is a separator")
	assert.Equal(t, []int{3, -4}, Ints("3 -4"))
	assert.Empty(t, Ints("none - -"))
}
