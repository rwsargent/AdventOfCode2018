package day24

import (
	"testing"
	"utils"
	"utils/aoctest"
)

func TestBuildGraph(t *testing.T) {
	graph := buildGraph(util.MustReadInput("inputs/day24_test.txt"))
	aoctest.AssertIntEquals(8, len(graph), t)

	edgeCount := 0
	for _, node := range graph {
		edgeCount += len(node.Neighbors)
	}
	aoctest.AssertIntEquals(16, edgeCount, t)
}

func TestBFS(t *testing.T) {
	graph := buildGraph(util.MustReadInput("inputs/day24_test.txt"))

	Dijkstra(graph)

	aoctest.AssertIntEquals(12, graph["10"].MaxWeight, t)
}

func TestDFS(t *testing.T) {
	graph := buildGraph(util.MustReadInput("inputs/day24_test.txt"))

	var max = 0

	DFS(graph, graph["0"], &max)

	t.Log("Max", max)
}

func TestRunPartOne(t *testing.T) {
	graph := buildGraph(util.MustReadInput("inputs/day24.txt"))

	var max = 0

	DFS(graph, graph["0"], &max)

	t.Log("Max:", max)
}

func TestRunDFSWithLongest(t *testing.T) {
	graph := buildGraph(util.MustReadInput("inputs/day24.txt"))

	var max = 0
	var longest = 0

	FindLongestDFS(graph, graph["0"], &max, 0, &longest)

	t.Log("Max:", max, "Longest:", longest)
}
