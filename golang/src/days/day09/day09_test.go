package main

import (
	"testing"
	"utils/aoctest"
)

func TestOpenClose(t *testing.T) {
	score, _ := streamReader("{}")
	aoctest.AssertIntEquals(1, score, t)
}


func TestThreeNested(t *testing.T) {
	score, _ := streamReader("{{{}}}")
	aoctest.AssertIntEquals(6,score , t)
}

func TestThreeGroupScoreFive(t *testing.T) {
	score, _ := streamReader("{{},{}}")
	aoctest.AssertIntEquals(5, score, t)
}

func TestLargeGroups(t *testing.T) {
	score, _ := streamReader("{{{},{},{{}}}}")
	aoctest.AssertIntEquals(16, score, t)
}

func TestOneGroupAsInGarbage(t *testing.T) {
	score, garbage := streamReader("{<a>,<a>,<a>,<a>}")
	aoctest.AssertIntEquals(1, score, t)
	aoctest.AssertIntEquals(4, garbage, t)
}

func TestManyGroupsManyGarbage(t *testing.T) {
	score, garbage := streamReader("{{<ab>},{<ab>},{<ab>},{<ab>}}")
	aoctest.AssertIntEquals(9, score, t)
	aoctest.AssertIntEquals(8, garbage, t)
}

func TestBangsInGarbage(t *testing.T) {
	score, garbage := streamReader("{{<!!>},{<!!>},{<!!>},{<!!>}}")
	aoctest.AssertIntEquals(9, score, t)	
	aoctest.AssertIntEquals(0, garbage, t)
}

func TestBangsEscapingGarbage(t *testing.T) {
	score, garbage := streamReader("{{<a!>},{<a!>},{<a!>},{<ab>}}")
	aoctest.AssertIntEquals(3, score, t)
	aoctest.AssertIntEquals(17, garbage, t)
}
