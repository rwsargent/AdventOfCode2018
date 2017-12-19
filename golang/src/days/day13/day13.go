package main

import (
	"strings"
	"utils"
)

type Scanner struct {
	Depth, Direction, Position int
}

func (other *Scanner)copy() *Scanner {
	return &Scanner {
		Depth : other.Depth,
		Direction : other.Direction,
		Position : other.Position,
	}
}

func main  () {
	SolveSecond()
}

func SolveSecond() {
	delay := delaySimulation("inputs/day13.txt")
	println("Delay:", delay)
}

func SolveFirst() {
	cost := runSimulation("inputs/day13.txt")
	println("Cost: ", cost)
}

func redditSolution(filename string) int {
	scanners, _ := buildScanners(util.MustReadInput(filename))

	TopLoop:
	for count := 0; count < 122522400; count++ {
		for pos, value := range *scanners {
			if calcScanner(value.Depth, count + pos) == 0 {
				continue TopLoop
			}
		}
		return count
	}
	return -1
}


func calcScanner(height, time int) int {
	offset := time % ((height - 1) * 2)

	if offset > height -1  {
		return 	2 * (height - 1) - offset
	} else {
		return offset
	}
}

func delaySimulation(filename string) int {
	// brute force, baby!
	baseScanners, maxLayer := buildScanners(util.MustReadInput(filename))
	for delay := 1; delay <= 122522400; delay++ {
		moveScanners(baseScanners)

		scanners := copyScanners(baseScanners)
		cost := 0
		for layer := 0; layer <= maxLayer; layer++ {
			cost += calculateCost(scanners, layer)
			moveScanners(scanners)
		}
		
		if cost == 0 {
			return delay
		}
	}
	return -1;
}

func returnToStart(scanners *map[int]*Scanner) bool {
	for _, scanner := range *scanners {
		if scanner.Position != 1 {
			return false
		}
	}
	return true
}

func runSimulation(filename string) int {
	scanners, maxLayer := buildScanners(util.MustReadInput(filename))

	cost := 0
	for layer := 0; layer <= maxLayer; layer++ {
		cost += calculateCost(scanners, layer)
		moveScanners(scanners)
	}

	return cost
}

func calculateCost(scanners *map[int]*Scanner, layer int) int {
	if scanner, ok := (*scanners)[layer]; ok {
		if scanner.Position == 1 {
//			println("Collison on layer:", layer, "!") 
			return (scanner.Depth * layer)
		}
	}
	return 0
}

func moveScanners(scanners *map[int]*Scanner) {
	for _, scanner := range (*scanners) {
		if (scanner.Position == scanner.Depth && scanner.Direction == 1) || (scanner.Position == 1 && scanner.Direction == -1) {
			scanner.Direction *= -1
		}
		scanner.Position += scanner.Direction
	}
}

func buildScanners(lines []string) (*map[int]*Scanner, int){
	scanners := make(map[int]*Scanner)
	var maxLayer int
	for _, line := range lines {
		tokens := strings.Split(line, ":")
		layer := util.MustAtoi(tokens[0])
		
		if(layer > maxLayer) {
			maxLayer = layer
		}

		scanners[layer] = &Scanner{Direction : 1, Depth : util.MustAtoi(tokens[1]), Position : 1}
	}
	return &scanners, maxLayer
}

func copyScanners(src *map[int]*Scanner) *map[int]*Scanner {
	new := make(map[int]*Scanner)
	for key, value := range *src {
		new[key] = value.copy()
	}

	return &new
}
