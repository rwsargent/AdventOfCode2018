package day24

import (
	"utils"
	"strings"
	"sort"
)

type Node struct {
	Id string
	MaxWeight int
	Neighbors []*Edge
	Visited bool
}

type Edge struct {
	Src, Dst *Node
	Weight int
	Visited bool
}

//Poor mans pq
type ByWeight []*Node
func (q ByWeight) Len() int           { return len(q) }
func (q ByWeight) Swap(i, j int)      { q[i], q[j] = q[j], q[i] }
func (q ByWeight) Less(i, j int) bool { return q[i].MaxWeight > q[j].MaxWeight}

func (this *Node) UpdateMaxWeight(newWeight int) bool {
	if(newWeight > this.MaxWeight) {
		this.MaxWeight = newWeight
		return true
	}
	return false
}

func FindLongestDFS(graph map[string]*Node, current *Node, max *int, length int, longest *int) {
	for _, edge := range current.Neighbors {
		if !edge.Visited {
			edge.Visited = true
			next := edge.Dst
			if next == current {
				next = edge.Src
			}
			oldWeight := next.MaxWeight
			next.MaxWeight = current.MaxWeight + edge.Weight

			if(length > *longest ) {
				*longest = length
				*max = next.MaxWeight
			} else if (length == *longest ) {
				if next.MaxWeight > *max {
					*max = next.MaxWeight
				}
			}
			FindLongestDFS(graph, next, max, length + 1, longest)
			next.MaxWeight = oldWeight
			edge.Visited = false
		}
	}	
}

func DFS(graph map[string]*Node, current *Node, max *int) {
	for _, edge := range current.Neighbors {
		if !edge.Visited {
			edge.Visited = true
			next := edge.Dst
			if next == current {
				next = edge.Src
			}
			oldWeight := next.MaxWeight
			next.MaxWeight = current.MaxWeight + edge.Weight
			if(next.MaxWeight > *max) {
				*max = next.MaxWeight
			}
			DFS(graph, next, max)
			next.MaxWeight = oldWeight
			edge.Visited = false
		}
	}
}

func Dijkstra(graph map[string]*Node) {
	q := make([]*Node, 0)
	graph["0"].MaxWeight = 0
	q = append(q, graph["0"])
	for len(q) > 0 {
		// pop the top
		current := q[0]
		q = q[1:]

		current.Visited = true
		for _, edge := range current.Neighbors {
			next := edge.Dst
			if next == current {
				next = edge.Src
			}
			if !next.Visited {
				next.UpdateMaxWeight(current.MaxWeight + edge.Weight)
				println("Visiting", next.Id, "max Weight", next.MaxWeight)
				q = append(q, next)
				sort.Sort(ByWeight(q))
			}
		}
	}
}

func buildGraph(lines []string) map[string]*Node {
	graph := make(map[string]*Node)
	for _, line := range lines {
		tokens := strings.Split(line, "/")
		src := graph[tokens[0]]
		if src == nil {
			src = &Node{tokens[0], 0, make([]*Edge, 0), false}
			graph[src.Id] = src
		}

		dest := graph[tokens[1]]
		if dest == nil {
			dest = &Node{tokens[1], 0, make([]*Edge, 0), false}
			graph[dest.Id] = dest
		}

		edge := &Edge{src, dest, util.MustAtoi(tokens[0]) + util.MustAtoi(tokens[1]), false}
		src.Neighbors = append(src.Neighbors, edge)
		dest.Neighbors = append(dest.Neighbors, edge)
	}
	return graph
}
