package day20

import (
	"testing"
	"utils/aoctest"
)

func TestPartOne(t *testing.T) {
	t.Skip()
	particles := ParseInput("inputs/day20.txt")
	closest := RunSimulation(particles, 500000)
	t.Log(closest)
}

func TestExample(t *testing.T) {
	t.Skip()
	particles := ParseInput("inputs/day20_test.txt")
	closest := RunSimulation(particles, 3)
	t.Log(closest)

	aoctest.AssertIntEquals(0, closest.Id, t)
}

func TestPartTwo(t *testing.T) {
	particles := ParseInput("inputs/day20.txt")
	count := RemoveSimulation(particles, 40)
	t.Log("Count:", count)
}

func TestMinIterCount(t *testing.T) {
	var count, iter = 0, 1
	for count != 567 {
		particles := ParseInput("inputs/day20.txt")
		count = RemoveSimulation(particles, iter)
		iter++
	}
	t.Log(iter)
}

func TestNaieve(t *testing.T) {
	particles := ParseInput("inputs/day20.txt")
	count := RemoveSimulationNaieve(particles, 400)
	aoctest.AssertIntEquals(567, count, t)
}
