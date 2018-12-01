package day19

import (
	"testing"
	"utils"
)

func TestExample(t *testing.T) {
	output := FollowPath(util.MustReadInput("inputs/day19_test.txt"))
	if output != "ABCDEF" {
		t.Error("Output:", output)
	}
}


func TestPartOne(t *testing.T) {
	output := FollowPath(util.MustReadInput("inputs/day19.txt"))
	t.Log("Output:", output)
}
