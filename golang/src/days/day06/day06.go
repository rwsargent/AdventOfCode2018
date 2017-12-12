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
	line, err := util.ReadInput("inputs/day06.txt")
	if(err != nil) {
		fmt.Println("Something went wrong!", err);
		return 
	}
	blockamount := strings.Split(line[0], "\t")
	blocks := util.StringsToInts(blockamount)
	fmt.Printf("Part 1: %d\n", distributAlgorithm(blocks))
}

func distributAlgorithm(blocks []int) int {
	var start, count int
	seen := make(map[string]bool)
	_, start = util.MaxValueAndIdx(blocks)
	for {
		start = distribute(blocks, start)
		if see, _ := seen[blockToKey(blocks)]; see == false {
			return count
		}
		count++
		seen[blockToKey(blocks)] = true
	}
}

func distribute(blocks []int, start int) int {
	amount := blocks[start]
	var maxIdx = start
	blocks[start] = 0
	var cursor = start + 1
	for ; amount > 0 ; amount-- {
		blocks[cursor]++
		if blocks[cursor] > blocks[maxIdx] {
			maxIdx = cursor
		}
		cursor = (cursor  + 1 ) % len(blocks)
	}
	return maxIdx
}

func blockToKey(blocks []int) string {
	return ""
}
