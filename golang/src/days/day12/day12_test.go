package main

import (
	"testing"
	"utils/aoctest"
)

func TestSupplied(t *testing.T) {
	graph := buildGraph("inputs/day12_test.txt")
	size := sizeOfGraph(graph)

	aoctest.AssertIntEquals(6, size, t)
}


func TestBuildGraph(t *testing.T) {
	graph := buildGraph("inputs/day12_test.txt")
	sizeOfGraph := len(graph)
	aoctest.AssertIntEquals(7, sizeOfGraph, t)
	aoctest.AssertIntEquals(3, len(graph["2"].Neighbors), t)
}
