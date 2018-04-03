package main

import (
	"utils"
	"strings"
)

func main() {
	//	SolveFirst()
	SolveSecond()
}

var dancers_const = []string {"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"}

func SolveFirst() {
	moves := strings.Split(util.MustReadInput("inputs/day16.txt")[0], ",")
	result := dance(moves, dancers_const)
	println(strings.Join(result, ""))
}

func SolveSecond() {
	moves := strings.Split(util.MustReadInput("inputs/day16.txt")[0], ",")
	var dancers = dancers_const

	memo := make(map[string]int)
	count := 0
	for ; count < 1000000000; count++ {
		dancers = dance(moves, dancers)
		if _, ok := memo[strings.Join(dancers, "")]; ok {
			break;
		}
		memo[strings.Join(dancers, "")] = count
	}
	println("Length:", len(memo))

	billionth := 1000000000 % count
	for key, value := range memo {
		if value == billionth-1 {
			println("Billionith permutation:", key)
		}
	}
	
}

func dance(moves []string, dancers []string) []string {
	for _, move := range moves {
		switch string(move[0]) {
		case "s" :
			dancers = spin(dancers, util.MustAtoi(move[1:]))
		case "x":
			nums := strings.Split(move[1:], "/")
			exchange(dancers, util.MustAtoi(nums[0]), util.MustAtoi(nums[1]))
		case "p":
			partners := strings.Split(move[1:], "/")
			partner(dancers, partners[0], partners[1])
		}
	}

	return dancers
}

func spin(dancers []string, length int) []string {
	front := dancers[:len(dancers) - length]
	back := dancers[len(dancers) - length:]
	
	return append(back, front...)
}

func exchange(dancers []string, a, b int) {
	dancers[a], dancers[b] = dancers[b], dancers[a]
}

func partner(dancers []string, a, b string) {
	aIdx := getIndex(dancers, a)
	bIdx := getIndex(dancers, b)
	exchange(dancers, aIdx, bIdx)
}


func getIndex(dancers []string, el string) int {
	for idx, dance := range dancers {
		if dance == el {
			return idx
		}
	}
	panic("Not here!")
}
