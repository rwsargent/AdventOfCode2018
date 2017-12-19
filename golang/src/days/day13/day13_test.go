package main

import (
	"testing"
	"utils/aoctest"
	"utils"
	"fmt"
)

func TestExample(t *testing.T) {
	cost := runSimulation("inputs/day13_test.txt")
	aoctest.AssertIntEquals(24, cost, t)
}


func TestDelay(t *testing.T) {
	delay := delaySimulation("inputs/day13_test.txt")
	aoctest.AssertIntEquals(10, delay, t)
}

func TestCurrentState(t *testing.T) {
	scanners, _ := buildScanners(util.MustReadInput("inputs/day13_test.txt"))
	for iter := 0; iter < 4; iter++ {
		moveScanners(scanners)
	}

	for key, value := range *scanners {
		fmt.Printf("%d: %+v\n", key, *value)
	}
	fmt.Println()
}

func TestReddit(t *testing.T) {
	delay := redditSolution("inputs/day13.txt");
	aoctest.AssertIntEquals(10, delay, t)
}
