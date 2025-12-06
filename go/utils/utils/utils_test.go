package utils

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSum(t *testing.T) {
	t.Run("Summarizes numbers", func(t *testing.T) {
		expected := 20
		actual := Sum([]int{12, 1, 7})

		assert.Equal(t, expected, actual)
	})
}
