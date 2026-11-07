package pq

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestQueueOrder(t *testing.T) {
	q := New(func(a, b int) bool { return a < b })

	for _, v := range []int{5, 1, 4, 2, 3} {
		q.Push(v)
	}

	assert.Equal(t, 5, q.Len())

	got := []int{}
	for q.Len() > 0 {
		got = append(got, q.Pop())
	}

	assert.Equal(t, []int{1, 2, 3, 4, 5}, got)
}

func TestQueueMaxHeapAndStructs(t *testing.T) {
	type job struct{ prio int }

	q := New(func(a, b job) bool { return a.prio > b.prio })
	q.Push(job{1})
	q.Push(job{9})

	assert.Equal(t, job{9}, q.Pop())
	assert.Equal(t, job{1}, q.Peek())
}
