package day19

import (
	"bytes"
	"fmt"
)

type Direction int

const (
	N Direction = iota
	E
	S
	W
)

type Cursor struct {
	Row, Col int
	Direction Direction
}

func (this *Cursor)Advance() {
	switch this.Direction {
	case N:
		this.Row--
	case E:
		this.Col++
	case S:
		this.Row++
	case W:
		this.Col--
	}
}

func FollowPath(graph []string) string {
	var startIdx int
	var stringBuilder bytes.Buffer
	for idx, char := range graph[0] {
		if char != ' ' {
			startIdx = idx
			break
		}
	}

	cursor := Cursor{0, startIdx, S}
	stepCount := 0
	loop:
	for {
		char := graph[cursor.Row][cursor.Col]
		switch char {
		case ' ':
			fmt.Println("We've reached the end!")
			break loop;
		case '-', '|':
			stepCount++
		case '+':
			if cursor.Direction == N || cursor.Direction == S {
				if InBounds(cursor.Row, cursor.Col+1, graph) && graph[cursor.Row][cursor.Col+1] != ' '{
					cursor.Direction = E
				} else {
					cursor.Direction = W
				}
			} else if cursor.Direction == E || cursor.Direction == W {
				if InBounds(cursor.Row-1, cursor.Col, graph) && graph[cursor.Row-1][cursor.Col] != ' '{
					cursor.Direction = N
				} else {
					cursor.Direction = S
				}
			}
			stepCount++
		default:
			if char >= 'A' && char <= 'Z' {
				stringBuilder.WriteByte(char)
				stepCount++
			} else {
				println("Unexpected char:", char)
			}
		}
		cursor.Advance()
	}

	fmt.Println("Stepcount:", stepCount)

	return stringBuilder.String()
}


func InBounds(row, col int, graph []string) bool {
	if row < 0 || row >= len(graph) {
		return false
	} else if col < 0 || col >= len(graph[0]) {
		return false
	}
	return true
}
