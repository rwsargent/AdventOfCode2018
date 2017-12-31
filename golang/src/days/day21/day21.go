package day21

import (
	"bytes"
	"strings"
)

type Chunk []string


func (this *Chunk) Flatten() string {
	var buf bytes.Buffer
	for _ ,row := range *this {
		buf.WriteString(row)
		buf.WriteRune('/')
	}

	return buf.String()
}

func NewChunk(flattened string) Chunk {
	return strings.Split(flattened, "/")
}

func (this Chunk)Rotate() Chunk {
	
}
