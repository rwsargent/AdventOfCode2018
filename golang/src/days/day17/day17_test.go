package day17

import (
	"testing"
	"utils/aoctest"
)

func TestExample(t *testing.T) {
	t.Skip()
	output := SpinLock(3, 1000000)
	aoctest.AssertIntEquals(638, output, t);
}

func TestPartOne(t *testing.T) {
	t.Skip()
	t.Log(SpinLock(382, 2017))
}

func TestPartTwo(t *testing.T) {
	t.Log(BetterSpinLock(382, 50000000))
}

