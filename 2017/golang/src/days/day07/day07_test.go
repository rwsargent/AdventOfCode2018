package main

import (
	"testing"
	"utils/aoctest"
)

/*
func TestTopProgram(t *testing.T) {
	progs := topProgram("inputs/test_day07.txt")
	t.Log(progs)
}*/

func TestBalance(t *testing.T) {
	programs := parseInput("inputs/test_day07.txt")
	amount := unbalanced("tknk", &programs)
	aoctest.AssertIntEquals(60, amount, t)
	
}

