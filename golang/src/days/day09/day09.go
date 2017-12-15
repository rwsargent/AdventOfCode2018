package main

import (
	"utils/day09"
	"utils"
	"fmt"
)

func main() {
	SolveFirst()
}

func SolveFirst() {
	lines, _ := util.ReadInput("inputs/day09.txt")
	score, garbage := streamReader(lines[0])
	fmt.Println("Group score: ", score)
	fmt.Println("Garbage: ", garbage)
}

func streamReader(stream string) (int, int) {
	stack := make(day09.Stack,0)
	score := 0
	garbage := 0
	var inGarbage bool
	for idx := 0; idx < len(stream); idx++ {
		var char = stream[idx]
		var el *day09.StreamElement
		if char == '!' { // skip next char
			idx++
			continue
		}
		if char == '>' {
			inGarbage = false
		} else if char == '<' {
			if(inGarbage) {
				garbage++
			}
			inGarbage = true
		} else if char == '{' {
			if(!inGarbage) {
				stack = stack.Push(day09.StreamElement{'{', 0})
			} else {
				garbage++
			}
		} else if char == '}' {
			if(!inGarbage) {
				stack, el = stack.Pop()
				if(!stack.IsEmpty()) {
					next := stack.Peek()
					next.GroupCount += 1 + el.GroupCount
				}
				score += el.GroupCount + 1
			} else {
				garbage++
			}
		} else {
			if(inGarbage) {
				garbage++
			}
			continue
		}
	}
	
	return score, garbage
}
