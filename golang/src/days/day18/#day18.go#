package day18

import (
	"utils"
	"strings"
)
type Instruction func(string, string)


func Execute(instructions []string) int {
	var played int
	regs := make(map[string]int)
	programCounter := 0
	
	var ops = map[string]Instruction {
	}

	for {
		instruction := strings.Split(instructions[programCounter], " ")
		switch instruction[0] {
		case "snd": 
			played = util.MustAtoi(instruction[1])
		},
		case "set": 
			regs[X] = util.MustAtoi(Y)
		},
		"add" : func(X, Y string)  {
			regs[X]+= util.MustAtoi(Y)
		},
		"mul" : func(X, Y string)  {
			regs[X]*= util.MustAtoi(Y)
		},
		"mod" : func(X, Y string)  {
			regs[X] %= util.MustAtoi(Y)
		},
		"rcv" : func(X, Y string)  {
			if util.MustAtoi(X) == 0 {
				panic(played)
			}
		},
		"jgz" : func(X, Y string) {
			if util.MustAtoi(X) > 0 {
				programCounter += util.MustAtoi(Y)
			}
		},
	}
	
	return played
}



