package main

import (
	"testing"
	"fmt"
)


func TestFirst(t *testing.T) {
	steps := SolveFirst([]int{0, 3, 0, 1, -3});
	if steps != 5 {
		t.Error(fmt.Sprintf("Steps was %d, should've been 5", steps))
	}
}

func TestSecond(t *testing.T) {
	steps := SolveSecond([]int{0, 3, 0, 1, -3})
	if steps != 10 {
		t.Error(fmt.Sprintf("Steps was %d, should've been 10\n", steps))
	}
}
