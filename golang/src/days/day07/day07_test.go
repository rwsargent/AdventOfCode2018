package main

import (
	"testing"
)

func TestTopProgram(t *testing.T) {
	progs := topProgram("inputs/test_day07.txt")
	t.Log(progs)
}

