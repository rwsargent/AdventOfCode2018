package main

import (
	"fmt"
	"regexp"
	"strings"
	"strconv"
	"utils"
)

func main() {
	PartOne()
}

type Program struct {
	Name string
	Weight int
	Below []string
}

var Pattern = regexp.MustCompile(`([a-z]+) \((\d+)\)( -> (([a-z]+, )*[a-z]+))?`)

func PartOne() {
	programs := topProgram("inputs/day07.txt")
	fmt.Println("Last remaining program: ")
	fmt.Println(programs)	

}

func topProgram(filename string) map[string]Program {
	programs := parseInput(filename)
	for _, program := range programs {
		for _, dep := range program.Below {
			fmt.Printf("Before size: %d\n", len(programs))
			remove(dep, programs)
			fmt.Printf("After size: %d\n", len(programs))
		}
	}

	return programs
}

func remove(name string, programs map[string]Program ) {
	if _, ok := programs[name]; !ok {
		return
	}
	for _, dependent := range programs[name].Below {
		remove(dependent, programs)
	}

	delete(programs, name);
}

func parseInput(filename string) map[string]Program {
	lines := util.MustReadInput(filename)
	var programs = make(map[string]Program)
	for _, line := range lines {
		program := parseLine(line);
		programs[program.Name] = program
	}

	return programs
}

func parseLine(line string ) Program {
	groups := Pattern.FindStringSubmatch(line)
	weight, err := strconv.Atoi(groups[2])
	if(err != nil) {
		panic(err)
	}
	program :=  Program {
		Name : groups[1],
		Weight : weight,
	}

	if(len(groups) > 2) {
		program.Below = strings.Split(groups[3], ", ")
	}

	return program
}
