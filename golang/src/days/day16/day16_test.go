package main

import (
	"testing"
	"fmt"
)


func TestExample(t *testing.T) {
	var dancers = []string {"a", "b", "c", "d", "e"}
	var moves = []string{"s1", "x3/4", "pe/b"}

	result := dance(moves, dancers)
	var expected = []string{"b", "a", "e", "d", "c"}
	for idx, val := range result {
		if val != expected[idx] {
			t.Error(fmt.Sprintf("Wrong value at %d. Expected %s, was %s\n", idx, expected[idx], val))
			break;
		}
	}
}
	
func TestSpin(t *testing.T) {
	var dancers = []string {"a", "b", "c", "d", "e"}
	res := spin(dancers, 3)

	var expected = []string {"c", "d", "e", "a", "b"}
	for idx, val := range res {
		if val != expected[idx] {
			t.Error(fmt.Sprintf("Wrong value at %d. Expected %s, was %s\n", idx, expected[idx], val))
			break;
		}
	}
}

func TestPartner(t *testing.T) {
	var dancers = []string {"a", "b", "c", "d", "e"}
	partner(dancers, "b", "d")

	var expected = []string {"a", "d", "c", "b", "e"}
	for idx, val := range dancers {
		if val != expected[idx] {
			t.Error(fmt.Sprintf("Wrong value at %d. Expected %s, was %s\n", idx, expected[idx], val))
			break;
		}
	}
}

func TestExchange(t *testing.T) {
	var dancers = []string {"a", "b", "c", "d", "e"}
	exchange(dancers, 1, 3)

	var expected = []string {"a", "d", "c", "b", "e"}
	for idx, val := range dancers {
		if val != expected[idx] {
			t.Error(fmt.Sprintf("Wrong value at %d. Expected %s, was %s\n", idx, expected[idx], val))
			break;
		}
	}
}

func TestSolveFirst(t *testing.T) {
	SolveFirst()
}
