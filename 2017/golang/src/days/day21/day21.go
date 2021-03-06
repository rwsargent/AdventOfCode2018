package day21

import (
	"bytes"
	"strings"
)

// ###/###/### -> 
type Chunk [][]rune


func (this Chunk) Flatten() string {
	var buf bytes.Buffer
	for _ ,row := range this {
		for _, col := range row {
			buf.WriteString(string(col))
		}
		buf.WriteRune('/')
	}

	return buf.String()
}

func NewChunk(flattened string) Chunk {
	rows := strings.Split(flattened, "/")
	chunk := make(Chunk, len(rows))
	for rIdx, row := range rows {
		chunk[rIdx] = []rune(row)
	}
	return chunk
}

func CountPixels(lines []string, iterations) int {
	rules := BuildRules(lines)
	picture := [][]rune{{'.', '#', '.'},{'.', '.', '#'}, {'#','#','#'}}
	newpic := allocateSize(calcnewsize(len(picture)))
	for iter:= 0; iter < iterations; ++iter {
		for roffset := 0; roffset < len(picture); roffset += 3 {
			for coffset := 0; coffset < len(picture[roffset]); c += 3 {
				
			}
		}
	}
		
	return 
}

func calcnewsize(size int) int {
	if size % 3 == 0 {
		return 4 * (size/3)
	} else if pic % 2 == 0 {
		return 3 * (size/2)
	}
}

func allocateSize(size int) [][]rune {
	ret := make([][]rune, size)
	for idx, _ := range ret {
		ret[idx] = make([]rune, size)
	}

	return ret
}

func BuildRules(lines []string) map[string]Chunk {
	rules := make(map[string]Chunk)
	
	for _, kvp := range lines {
		tokens := strings.Split(kvp, " => ")

		rule := NewChunk(tokens[0])
		result := NewChunk(tokens[1])
		size := len(rule[0])

		transforms := []func(int, int)(int, int) {
			func(i, j int) (int, int) {
				return i, j
			},
			func(i, j int) (int, int) {
				return j, i
			},
			func(i, j int) (int, int) {
				return size - j - 1, i
			},
			func(i, j int) (int, int) {
				return j, size - i -1
			},
			func(i, j int) (int, int) {
				return size - i - 1, j
			},
			func(i, j int) (int, int) {
				return i, size - j- 1
			},
			func(i, j int) (int, int) {
				return size - i - 1, size - j - 1
			},
			func(i, j int) (int, int) {
				return size - j - 1, size - i - 1
			},
		}

		for _, trans := range transforms {
			runes := make([][]rune, size)
			for idx := 0 ; idx < size; idx++ {
				runes[idx] = make([]rune, size)
			}

			transformation := Chunk(runes)
			
			for i, _ := range rule {
				for j, _ := range rule[i] {
					newi, newj := trans(i, j)
					transformation[i][j] = rule[newi][newj]
				}
			}

			rules[transformation.Flatten()] = result
		}
	}
	return rules
}
func AddToRules(rules map[string]Chunk, key string, result Chunk) {
	if old, ok := rules[key]; ok {
		if old.Flatten() != result.Flatten() {
			panic("Rule: " + key)
		}
	}
	rules[key] = result
}
