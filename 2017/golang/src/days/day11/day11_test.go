package main

import (
	"testing"
	"fmt"
)

func TestSuppliedNeNeNe(t *testing.T) {
	steps := []string{"ne","ne","ne"}
	dist := getDistance(steps)
	if(dist != 3) {
		t.Error(fmt.Sprintf("Expected 3, got %d", dist))
	}
}

func Testneneswsw(t *testing.T) {
	steps := []string{"ne","ne","sw","sw"}
	dist := getDistance(steps)
	if(dist != 0) {
		t.Error(fmt.Sprintf("Expected 0, got %d", dist))
	}
}
