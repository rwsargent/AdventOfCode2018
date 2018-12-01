package main

import (
	"testing"
	"utils/aoctest"
)

func TestReverseSubList(t *testing.T) {
	list := makeList(5)
	list = reverseSublist(list, 0, 3)
	t.Log(list)
	list = reverseSublist(list, 3, 4)
	t.Log(list)
	list = reverseSublist(list, 3, 1)
	t.Log(list)
}


func TestHash(t *testing.T) {
	list := []int{0, 1, 2, 3, 4}
	lengths := []int{3, 4, 1, 5}
	aoctest.AssertIntEquals(12, hash(lengths, list), t)
	t.Log(list)
}
