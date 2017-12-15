package main

import (
	"testing"
	"fmt"
)

func TestSupplied(t *testing.T) {
	graph := buildGraph("inputs/day12_test.txt")
	size := sizeOfGraph(graph)

	if size != 6 {
		t.Error(fmt.Sprintf("Expected 6, was %d", size))
	}
}
