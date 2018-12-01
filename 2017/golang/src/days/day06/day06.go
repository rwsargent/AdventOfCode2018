package main

import (
	"fmt"
	"utils"
	"strings"
)

func main() {
	SolveSecond()
}

func SolveFirst() []int {
	line, err := util.ReadInput("inputs/day06.txt")
	if(err != nil) {
		fmt.Println("Something went wrong!", err);
		return nil
	}
	blockamount := strings.Split(line[0], "\t")
	seen := make(map[string]bool)
	blocks := util.StringsToInts(blockamount)
	fmt.Printf("Part 1: %d\n", distributAlgorithm(blocks, seen))
	return blocks
}

func SolveSecond() {
	blocks := SolveFirst();
	seen := map[string]bool {
		blockToKey(blocks) : true,
	}
	
	fmt.Printf("Part 2: %d\n", distributAlgorithm(blocks, seen))
}

func distributAlgorithm(blocks []int, seen map[string]bool) int {
	var start, count int
	_, start = util.MaxValueAndIdx(blocks)
	for {
		start = distribute(blocks, start)
		count++
		key := blockToKey(blocks)
		if see, _ := seen[key]; see {
			return count
		}
		seen[key] = true
	}
	return count
}

func distribute(blocks []int, start int) int {
	var verbose = false
	amount := blocks[start]
	if verbose {
		fmt.Printf("Distribute %v (picking %d at %d) -> ", blocks, blocks[start], start)
	}
	blocks[start] = 0
	var cursor = (start + 1) % len(blocks)
	for ; amount > 0 ; amount-- {
		blocks[cursor]++
		cursor = ((cursor  + 1 ) % len(blocks))
	}
	var maxIdx = 0
	for idx := len(blocks) -1 ; idx >= 0; idx-- {
		if blocks[idx] >= blocks[maxIdx] {
			maxIdx = idx
		}
	}
	if verbose {
		fmt.Printf("%v\n", blocks);
	}
	return maxIdx
}

func blockToKey(blocks []int) string {
	return fmt.Sprintf("%v", blocks);
	/*var key string
	for _, block := range blocks {
		key += strconv.Itoa(block)
	}
	return key*/
}
