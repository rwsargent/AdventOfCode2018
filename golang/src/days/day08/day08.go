package main

import (
	"regexp"
	"strconv"
	"utils"
	"math"
	"fmt"
)

var Pattern = regexp.MustCompile(`([a-z]+) (inc|dec) (-?\d+) if ([a-z]+) ([<=!>]+) (-?\d+)`)

type Instruction struct {
	Register, TestRegister string
	Amount, TestValue int
	Test func(int, int)bool
}

var TESTS = map[string]func(int, int)bool {
	"!=" : func(a, b int) bool { return a != b },
	"==" : func(a, b int) bool { return a == b },
	">" : func(a, b int) bool{ return a > b },
	">=" : func(a, b int) bool{ return a >= b },
	"<" : func(a, b int) bool { return a < b },
	"<=" : func(a, b int) bool { return a <= b },
}

func main() {
	SolveFirst()
}

func SolveFirst() {
	sourceCode := util.MustReadInput("inputs/day08.txt")
	registers := runInstructions(sourceCode)
	var maxVal = math.MinInt32
	
	for _, value := range registers {
		if value > maxVal {
			maxVal = value
		}
	}

	fmt.Println("Max value: ", maxVal)
}

func runInstructions(instructions []string) map[string]int {
	registers := make(map[string]int)
	var highest int = math.MinInt32
	for _, source := range instructions {
		instruction := parseInstruction(source)

		if instruction.Test(registers[instruction.TestRegister], instruction.TestValue) {
			registers[instruction.Register] += instruction.Amount

			if registers[instruction.Register] > highest {
				highest = registers[instruction.Register]
			}
		}
	}

	fmt.Println("Heightest:", highest) 
	return registers
}


func parseInstruction(line string) Instruction {
	tokens := Pattern.FindStringSubmatch(line)

	amount, _ := strconv.Atoi(tokens[3])
	if tokens[2] == "dec" {
		amount = -amount
	}

	value, _ := strconv.Atoi(tokens[6])

	return Instruction {
		Register : tokens[1],
		Amount : amount,
		TestRegister : tokens[4],
		TestValue : value,
		Test : TESTS[tokens[5]],
	}
}
