package main

import (
	"testing"
	"utils/aoctest"
)

func TestJudgePairs(t *testing.T) {
	aoctest.AssertIntEquals(588, judge(65, 8921), t)
}

func TestGeneratorA(t *testing.T) {
	gen := Generator{65, 16807}
	aoctest.AssertIntEquals(1092455, gen.next(), t)
	aoctest.AssertIntEquals(1181022009, gen.next(), t)
	aoctest.AssertIntEquals(245556042, gen.next(), t)
}

func TestGeneratorB(t *testing.T) {
	gen := Generator{8921, 48271}
	aoctest.AssertIntEquals(430625591, gen.next(), t)
	aoctest.AssertIntEquals(1233683848, gen.next(), t)
	aoctest.AssertIntEquals(1431495498, gen.next(), t)
}


func TestJudgePicky(t *testing.T) {
	aoctest.AssertIntEquals(309, judgePicky(65, 8921), t)
}

func TestPickyA(t *testing.T) {
	gen := Generator{65, 16807}
	aoctest.AssertIntEquals(1352636452, gen.nextPicky(4), t)
	
}
