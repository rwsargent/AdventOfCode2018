package main

import (
	"fmt"
	"utils"
	"strconv"
)

func main() {
//	steps := SolveFirst(getInstructions("day05_input.txt"))
	//	fmt.Println("Part one steps: ", steps)
	steps := SolveSecond(getInstructions("day05_input.txt"))
	fmt.Println("Part two steps: ", steps)
}

func SolveFirst(instructions []int) int {
	var pos, counter int
	for pos < len(instructions) {
		current := pos
		pos += instructions[pos]
		instructions[current]++
		counter++
	}

	return counter
}

func SolveSecond(instructions []int) int {
	var pos, counter int
	for pos < len(instructions) {
		current := pos
		pos += instructions[pos]
		if instructions[current] > 2 {
			instructions[current]--
		} else {
			instructions[current]++
		}
		counter++
	}

	return counter
}


func getInstructions(path string) []int {
	lines, err := util.ReadInput(path)
	if(err != nil) {
		fmt.Println(err)
	}

	instructions := make([]int, len(lines), cap(lines))
	for idx, instruction := range lines {
		instructions[idx], err = strconv.Atoi(instruction)
		if(err != nil) {
			fmt.Println(err)
		}
	}

	return instructions
}
