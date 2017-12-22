package day18

import (
	"strings"
	"fmt"
	"strconv"
)


func Program(instructions []string, counter *int, receiver chan int, sender chan int, programId int) {
	regs := map[string]int{
		"p" : programId,
	}
	programCounter := 0
	for {
		cmd, X, Y := resolveInstruction(strings.Split(instructions[programCounter], " "), regs)
		switch cmd {
		case "snd":
			sender <- regs[X]
			*counter++
			fmt.Printf("Program %d has sent %d times\n", programId, *counter)
		case "set": 
			regs[X] = Y
		case "add": 
			regs[X]+= Y
		case "mul":
			regs[X]*= Y
		case "mod": 
			regs[X] %= Y
		case "rcv":
			regs[X] = <- receiver
		case "jgz":
			// hardcoded from puzzle input, sorry.
			if X == "1" || regs[X] > 0 {
				programCounter += Y
				continue
			}
		}
		programCounter++
	}
}

func Execute(instructions []string) int {
	regs := make(map[string]int)
	var played int
	programCounter := 0
	Program:
	for {
		cmd, X, Y := resolveInstruction(strings.Split(instructions[programCounter], " "), regs)
		fmt.Println(instructions[programCounter])
		switch cmd {
		case "snd": 
			played = regs[X]
		case "set": 
			regs[X] = Y
		case "add": 
			regs[X]+= Y
		case "mul":
			regs[X]*= Y
		case "mod": 
			regs[X] %= Y
		case "rcv": 
			if regs[X] != 0 {
				break Program
			}
		case "jgz":
			if regs[X] > 0 {
				programCounter += Y
				continue
			}
		}
		fmt.Println(regs)
		programCounter++
	}
	return played

}

func resolveInstruction(instruction []string, registers map[string]int) (string,string,int) {
	X := instruction[1]
	Y := 0
	if len(instruction) == 3 {
		if num, err := strconv.Atoi(instruction[2]); err == nil {
			Y = num
		} else {
			Y = registers[instruction[2]]
		}
	}
	return instruction[0], X, Y
}
