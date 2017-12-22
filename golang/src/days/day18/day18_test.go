package day18

import (
	"testing"
	"utils"
	"utils/aoctest"
)

func TestExample(t *testing.T) {
	aoctest.AssertIntEquals(4 ,Execute(util.MustReadInput("inputs/day18_test.txt")), t)
}

func TestPartOne(t *testing.T) {
	t.Log(Execute(util.MustReadInput("inputs/day18.txt")))
}

func TestPartTwo(t *testing.T) {
	p1ToP0 := make(chan int, 1024)
	p0ToP1 := make(chan int, 1024)
	join := make(chan int)
	instructions := util.MustReadInput("inputs/day18.txt")

	p1Counter := 0
	p0Counter := 0
	
	go Program(instructions, &p0Counter, p1ToP0, p0ToP1, 0)
	go Program(instructions, &p1Counter, p0ToP1, p1ToP0, 1)
	<- join 
}
