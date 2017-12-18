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
	PartTwo()
}

type Program struct {
	Name string
	Weight, TotalWeight int
	Below []string
}

var Pattern = regexp.MustCompile(`([a-z]+) \((\d+)\)( -> (([a-z]+, )*[a-z]+))?`)

func PartOne() {
	programs := topProgram("inputs/day07.txt")
	fmt.Println("Last remaining program: ")
	fmt.Println(programs)	
}

func PartTwo() {
	programs := parseInput("inputs/day07.txt")
	amount := unbalanced("vvsvez", &programs)
	fmt.Println("Unbalance amount: ", amount)
}

func topProgram(filename string) map[string]*Program {
	programs := parseInput(filename)
	for _, program := range programs {
		for _, dep := range program.Below {
			remove(dep, programs)
		}
	}

	return programs
}


/* This function has the lovely side effect of changing the programs
 * totaly weight, while returning any discrepencies in weight*/

func unbalanced(root string, programs *map[string]*Program) int {
	program, ok := (*programs)[root]
	if !ok {
		panic("Couldn't find: " + root)
	}
	dependees := make([]*Program, len(program.Below))
	var unbalance int
	for idx, dependee := range program.Below  {
		unbalance = unbalanced(dependee, programs)
		if( unbalance != 0) {
			return unbalance
		}
		dependees[idx] = (*programs)[dependee]
	}

	unbalance = balance(dependees)
	if unbalance == 0 {
		fmt.Println(root, "is balanced")
	}
	(*programs)[root].TotalWeight = (*programs)[root].Weight + sum(dependees)
	return unbalance
}

func sum(weights []*Program) int {
	var sum int
	for _, weight := range weights {
		sum += weight.TotalWeight
	}
	return sum
}

func balance(dependees []*Program) int {
	oddManOut := make(map[int][]*Program)
	for _, dependee := range dependees {
		oddManOut[dependee.TotalWeight] = append(oddManOut[dependee.TotalWeight], dependee)
	}
	
	//Found the outlier
	var solo, many *Program
	if len(oddManOut) == 2 {
		fmt.Println("Found an odd man out!")
		for _, value := range oddManOut {
			if len(value) > 1 {
				many = value[0]
			} else {
				solo = value[0]
				fmt.Println("Looks like", solo.Name, "is unbalanced")
			}
		}
		return solo.Weight + (many.TotalWeight - solo.TotalWeight)
	} else if len(oddManOut) > 2 {
		panic("ERROR: More than two odd men out!")
	}
	return 0
}


func remove(name string, programs map[string]*Program ) {
	if _, ok := programs[name]; !ok {
		return
	}
	for _, dependent := range programs[name].Below {
		remove(dependent, programs)
	}

	delete(programs, name);
}

func parseInput(filename string) map[string]*Program {
	lines := util.MustReadInput(filename)
	var programs = make(map[string]*Program)
	for _, line := range lines {
		program := parseLine(line);
		programs[program.Name] = &program
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
		Below : nil,
	}

	if(len(groups[4]) > 0) {
		program.Below = strings.Split(groups[4], ", ")
	}
	
	return program
}
