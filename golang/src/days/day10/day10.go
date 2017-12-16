package main

import (
	"fmt"
	"utils"
	"strings"
)
	
func main() {
	SolveFirst()
}

func SolveFirst() {
	lengths := parseInput("inputs/day10.txt")
	list := makeList(256)
	fmt.Println("Hash: ", hash(lengths, list))
}

func makeList(size int) []int{
	list := make([]int, size)
	for idx := range list {
		list[idx] = idx
	}
	return list
}

func hash(lengths []int, list []int) int {
	var current, skip int = 0, 0
	for _, length := range lengths {
		list = reverseSublist(list, current, length)
		current = (current + length + skip) % len(list)
		skip++
		fmt.Println("list:" , list)
		fmt.Println("current:" , current)
		fmt.Println("skip:" , skip)
	}

	return list[0] * list[1]
}

func reverseSublist(list []int, current int, length int) []int {
	for idx := 0; idx < length / 2; idx++ {
		list[(idx + current) % len(list)], list[((current + length - 1) - idx) % len(list)] = list[((current + length - 1) - idx) % len(list)], list[(idx + current) % len(list)]
	}
	return list
}

func parseInput(filename string) []int {
	lines, _ := util.ReadInput("inputs/day10.txt")
	return util.StringsToInts(strings.Split(lines[0], ","))
}
