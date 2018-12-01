package day17

import (
	"fmt"
	"bytes"
	"strconv"
)

var lastItem int  = -1
func SpinLock(step int, size int) int {
	buffer := make([]int, 1, size+1)
	var cursor = 0
	for count := 1; count <= size; count++ {
		cursor = (cursor + step) % len(buffer)
		cursor++
		buffer = append(buffer, 0)
		copy(buffer[cursor+1:], buffer[cursor:])
		buffer[cursor] = count
	}

	return buffer[cursor + 1]
}

func BetterSpinLock(step int, size int) int {
	var cursor = 0
	var relevantElement = 0
	for count := 1; count <= size; count++ {
		cursor = (cursor + step) % count
		cursor++
		if cursor == 1 {
			relevantElement = count
		}
	}
	return relevantElement
}

func printFirst(buffer []int, cursor, row int) {
	if (cursor == 1) {
		fmt.Printf("%d: %d (%d)\n", row, buffer[0], buffer[1])
	}
}

func printInsertLocation(row, cursor int) {
	println(row, ":", cursor)
}


func printBuffer(buffer *[]int, cursor, row int) {
	var stringBuilder bytes.Buffer
	print(row, ": ")
	for idx, value := range *buffer {
		if idx == cursor {
			stringBuilder.WriteString("(")
			stringBuilder.WriteString(strconv.Itoa(value))
			stringBuilder.WriteString(") ")
		} else {
			stringBuilder.WriteString(strconv.Itoa(value) + " ")
		}
	}
	println(stringBuilder.String())
}
