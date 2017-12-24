package day22

import (
	"testing"
	"utils/aoctest"
)

func TestExample(t *testing.T) {
	nodes, virus := InitializeBoard("inputs/day22_test.txt")
	aoctest.AssertIntEquals(1, virus.Coord.Col, t)
	aoctest.AssertIntEquals(1, virus.Coord.Row, t)
	aoctest.AssertIntEquals(2, len(nodes), t)

	infected := Simulation(nodes, virus, 70)
	aoctest.AssertIntEquals(0, virus.DirectionIdx, t)
	aoctest.AssertIntEquals(41, infected, t)
}

func TestExampleLarge(t *testing.T) {
	nodes, virus := InitializeBoard("inputs/day22_test.txt")
	infected := Simulation(nodes, virus, 10000)

	aoctest.AssertIntEquals(5587, infected, t)
}

func TestPartOne(t *testing.T) {
	nodes, virus := InitializeBoard("inputs/day22.txt")
	infected := Simulation(nodes, virus, 10000)

	t.Log("Infected:", infected)
}

func TestExampleEvolved(t *testing.T) {
	nodes, virus := InitializeEvolvedBoard("inputs/day22_test.txt")
	infected := EvolvedSimulation(nodes, virus, 100)
	aoctest.AssertIntEquals(26, infected, t)
}

func TestPartTwo(t *testing.T) {
	nodes, virus := InitializeEvolvedBoard("inputs/day22.txt")
	infected := EvolvedSimulation(nodes, virus, 10000000)

	t.Log("Infected:", infected)
}
