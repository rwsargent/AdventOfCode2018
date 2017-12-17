package main

import (
	"fmt"
	"utils"
	"regexp"
	"strings"
)

func main () {
	SolveFirst()
	SolveSecond()
}

var Pattern = regexp.MustCompile(`(\d+) <-> ((\d+)(, \d+)*)`)

type Pipe struct {
	ID string
	Neighbors []string
	Visited bool
	Label int
}

func SolveFirst() {
	fmt.Println("Connected: ", sizeOfGraph(buildGraph("inputs/day12.txt")))
}
func SolveSecond() {
	fmt.Println("Group count:: ", numberOfGroups(buildGraph("inputs/day12.txt")))
}

func buildGraph(filename string) map[string]*Pipe {
	graph := make(map[string]*Pipe)

	lines, _ := util.ReadInput(filename)
	for _, line := range lines {
		pipe := parseLine(line)
		graph[pipe.ID] = &pipe
	}
	
	return graph
}

func sizeOfGraph(pipes map[string]*Pipe) int {
	counter := 1
	(*pipes["0"]).Visited = true
	dfs((*pipes["0"]).Neighbors, pipes, &counter)
	return counter
}

func numberOfGroups(graph map[string]*Pipe) int {
	var total, groupCount int
	
	for _, pipe := range graph {
		if !(*pipe).Visited {
			groupCount++
			dfs((*pipe).Neighbors, graph, &total) // flood fill from this starting point
		}
	}
	fmt.Println("total: ", total)
	return groupCount
}

func dfs(neighbors []string, graph map[string]*Pipe, counter *int) {
	for _, n := range neighbors {
		pipe := graph[n]
		if !pipe.Visited {
			(*counter)++
			pipe.Visited = true
			dfs(pipe.Neighbors, graph, counter)
		}
	}
}

func parseLine(line string) Pipe {
	groups := Pattern.FindStringSubmatch(line)
	
	pipe := Pipe {
		ID : groups[1],
		Neighbors : strings.Split(groups[2], ", "),
	}

	return pipe
}
