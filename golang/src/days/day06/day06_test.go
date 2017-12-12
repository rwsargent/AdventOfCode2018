package main

import "testing"

func TestPartOne(t *testing.T) {
	newStart := distribute([]int{0, 2, 7, 0}, 0)
	if newStart != 1 {
		t.Error("Wrong new start")
	}
}
