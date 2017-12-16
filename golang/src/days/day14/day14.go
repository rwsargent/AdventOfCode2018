package main

import (
	"utils/hash"
	"fmt"
)

func main () {
	SolveFirst()
	SolveSecond()
}

func SolveFirst() {
	fmt.Println("Squares used: ", usedBlock("wenycdww"))
}

func SolveSecond() {
	fmt.Println("Reginos used: ", usedRegions("wenycdww"))
}

func usedBlock(hashprefix string) int {
	var used int
	for idx := 0; idx < 128; idx++ {
		input := fmt.Sprintf("%s-%d", hashprefix, idx)
		bytes := hash.KnotHash(input)
		used += bitsOn(bytes)
	}

	return used
}

func bitsOn(bytes [16]byte)  int {
	var count int

	for _, byte := range bytes {
		b := byte
		for b != 0 {
			if b & 1 == 1 {
				count++
			}
			b >>= 1
		}
	}
	
	return count
}

//Just a flood fill
func usedRegions(hashprefix string) int {
	var regions int

	field := [128][128]bool {}
	
	for row := 0; row < 128; row++ {
		input := fmt.Sprintf("%s-%d", hashprefix, row)
		bytes := hash.KnotHash(input)
		fillBits(&field[row], bytes)
	}

	for row := range field {
		for col := range field[row] {
			if(field[row][col]) {
				regions++
				dfs(row, col, &field)
			}
		}
	}

	return regions
}

func dfs(row, col int, graph *[128][128]bool) {
	if row >= 128 || row < 0 || col >= 128 || col < 0 {
		return
	}

	if(!graph[row][col]) {
		return 
	}
	graph[row][col] = false // mark as visited
	// search further!
	dfs(row+1, col, graph)
	dfs(row, col+1, graph)
	dfs(row, col-1, graph)
	dfs(row-1, col, graph)
}

func fillBits(row *[128]bool, bytes [16]byte) {
	for bOffset := 0; bOffset < 16; bOffset++ {
		bite := bytes[bOffset]
		mask := byte(128) // 1000 0000 in binary
		for bit := 0; bit < 8; bit++ {
			if (bite & mask) > 0 {
				(*row)[(bOffset * 8) + bit] = true
			}
			mask >>= 1
		}
	}
}
