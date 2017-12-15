package main

import (
	"utils"
	"regexp"
	"strings"
)

func main () {
	
}

var Pattern = regexp.MustCompile(`(\d+) <-> ((\d+)(, \d+)*)`)

type Pipe struct {
	ID string
	Neighbors []string
	Visited bool
}

func buildGraph(filename string) map[string]Pipe {
	graph := make(map[string]Pipe)

	lines, _ := util.ReadInput(filename)
	for _, line := range lines {
		pipe := parseLine(line)
		graph[pipe.ID] = pipe
	}
	
	return graph
}

func sizeOfGraph(pipes map[string]Pipe) int {
	counter := 1
	dfs(pipes["0"].Neighbors, pipes, &counter)
	return counter
}

func dfs(neighbors []string, graph map[string]Pipe, counter *int) {
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
