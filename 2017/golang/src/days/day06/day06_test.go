package main

import (
	"testing"
	"fmt"
)

func TestPartOne(t *testing.T) {
	newStart := distribute([]int{0, 2, 7, 0}, 2)
	if newStart != 1 {
		t.Error(fmt.Sprintf("Wrong new start: expected %d, was %d\n", 1, newStart))
	}
}

func TestPartAlgorithm(t *testing.T) {
	count := distributAlgorithm([]int{0, 2, 7, 0});
	if count != 5 {
		t.Error(fmt.Sprintf("Wrong count: expected %d, was %d\n", 5, count))
	}
}
