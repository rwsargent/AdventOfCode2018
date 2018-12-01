package main

import (
	"fmt"
	"utils"
	"strings"
	"strconv"
	"sort"
)

func main() {
	//	PartOne()
	PartTwo()
}

func PartOne() {
	lines, err := util.ReadInput("day04_input.txt")
	if(err != nil) {
		fmt.Println("Something went wrong")
		return 
	}
	valid := 0
	
	for _, line := range lines {
		fmt.Println(line)
		if isValid(line) {
			valid += 1
		}
	}
	fmt.Println("Part one: " + strconv.Itoa(valid))
}

func PartTwo() {
	lines, err := util.ReadInput("day04_input.txt")
	if(err != nil) {
		fmt.Println("Something went wrong")
		return 
	}
	valid := 0
	
	for _, line := range lines {
		fmt.Println(line)
		if isValidAnagram(line) {
			valid += 1
		}
	}
	fmt.Println("Part two: " + strconv.Itoa(valid))
}


func isValid(line string) bool {
	set :=  make(map[string]bool)
	words := strings.Split(line, " ")
	for _, word := range words {
		if set[word] {
			fmt.Println("Invalid on: " + word)
			return false
		}
		set[word] = true
	}
	return true
}

func isValidAnagram(line string) bool {
	set := make(map[string]bool)
	words := strings.Split(line, " ")
	for _, word := range words {
		sorted := sortString(word)
		if set[sorted] {
			return false
		}
		set[sorted] = true
	}
	return true
}

func sortString(w string) string {
    s := strings.Split(w, "")
    sort.Strings(s)
    return strings.Join(s, "")
}
