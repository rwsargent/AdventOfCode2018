package day21

import (
	"testing"
	"utils"
)

func TestBuildRules(t *testing.T) {
	rules := BuildRules(util.MustReadInput("inputs/day21.txt"))
	t.Log(rules)
}
