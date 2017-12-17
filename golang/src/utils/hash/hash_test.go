package hash

import (
	"testing"
	"utils/aoctest"
)

func TestKnotHash(t *testing.T) {
	out := HashToString(KnotHash(""))
	aoctest.AssertStrEquals("a2582a3a0e66e6e86e3812dcb672a272", out, t)

	out = HashToString(KnotHash("AoC 2017"))
	aoctest.AssertStrEquals("33efeb34ea91902bb2f59c9920caa6cd", out, t)

	out = HashToString(KnotHash("1,2,3"))
	aoctest.AssertStrEquals("3efbe78a8d82f29979031a4aa0b16a9d", out, t)
	
	out = HashToString(KnotHash("1,2,4"))
	aoctest.AssertStrEquals("63960835bcdc130f0b66d7ff4f6a5a8e", out, t)
}
