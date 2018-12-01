package main

import (
	"fmt"
	"math"
	"utils"
	"strings"
)

func main() {
	SolveFirst()
	SolveSecond()
}

type Cursor struct {
	X, Y, Z int
}

func SolveFirst() {
	dist := getDistance(readInput("inputs/day11.txt"))
	fmt.Println("Distance is: ", dist)		
}

func SolveSecond() {
	dist := getMaxDistance(readInput("inputs/day11.txt"))
	fmt.Println("Max Distance is: ", dist)		
}

func readInput(filename string) []string {
	s, _ := util.ReadInput(filename)
	
	return strings.Split(s[0], ",")
}

func getMaxDistance(steps []string) int {
	cursor := Cursor { 0, 0, 0}
	max := 0
	for _, step := range steps {
		switch step {
		case "n":
			cursor.Y++
			cursor.Z--
		case "s":
			cursor.Y--
			cursor.Z++
		case "ne":
			cursor.X++
			cursor.Z--
		case "sw":
			cursor.X--
			cursor.Z++
		case "nw":
			cursor.X--
			cursor.Y++
		case "se":
			cursor.X++
			cursor.Y--
		}
		dist := int((math.Abs(float64(cursor.X)) + math.Abs(float64(cursor.Y)) + math.Abs(float64(cursor.Z))) / 2)
		if dist > max {
			max = dist
		}
	}
	return max
}

func getDistance(steps []string) int {
	cursor := Cursor { 0, 0, 0}
	for _, step := range steps {
		switch step {
		case "n":
			cursor.Y++
			cursor.Z--
		case "s":
			cursor.Y--
			cursor.Z++
		case "ne":
			cursor.X++
			cursor.Z--
		case "sw":
			cursor.X--
			cursor.Z++
		case "nw":
			cursor.X--
			cursor.Y++
		case "se":
			cursor.X++
			cursor.Y--
		}
	}
	dist := int((math.Abs(float64(cursor.X)) + math.Abs(float64(cursor.Y)) + math.Abs(float64(cursor.Z))) / 2)
	return dist
}
