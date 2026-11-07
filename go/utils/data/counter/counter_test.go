package counter

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCounter(t *testing.T) {
	c := New("a", "b", "a")

	assert.Equal(t, 2, c["a"])
	assert.Equal(t, 1, c["b"])
	assert.Equal(t, 0, c["z"])

	c.Add("b")
	c.AddN("z", 5)

	assert.Equal(t, 2, c["b"])
	assert.Equal(t, 5, c["z"])
	assert.Equal(t, 9, c.Total())
}

func TestMostAndLeastCommon(t *testing.T) {
	c := New("a", "b", "b", "c", "c", "c")

	key, n := c.MostCommon()
	assert.Equal(t, "c", key)
	assert.Equal(t, 3, n)

	key, n = c.LeastCommon()
	assert.Equal(t, "a", key)
	assert.Equal(t, 1, n)

	_, n = New[string]().MostCommon()
	assert.Equal(t, 0, n)
}
