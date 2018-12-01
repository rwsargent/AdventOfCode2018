package day09

type StreamElement struct {
	Tag rune
	GroupCount int
}

type Stack []StreamElement

func (s Stack) Push(el StreamElement) Stack {
    return append(s, el)
}

func (s Stack) Peek() *StreamElement {
	return &s[len(s) - 1]
}

func (s Stack) Pop() (Stack, *StreamElement) {
    // FIXME: What do we do if the Stack is empty, though?
    l := len(s)
	if l == 0 {
		panic("Stack empty!")
	}
    return  s[:l-1], &s[l-1]
}

func (s Stack) IsEmpty() bool {
	return len(s) == 0
}
