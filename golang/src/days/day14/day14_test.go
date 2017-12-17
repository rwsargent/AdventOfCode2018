package main

import (
	"testing"
	"utils/aoctest"
)

func TestFlqrgnkx(t *testing.T) {
	aoctest.AssertIntEquals(8108, usedBlock("flqrgnkx"), t)
}

func TestFlqrgnkxRegion(t *testing.T) {
	aoctest.AssertIntEquals(1242,usedRegions("flqrgnkx"), t)
}
