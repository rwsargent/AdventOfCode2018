package aoctest


import (
	"testing"
	"fmt"
)

func AssertIntEquals(expected, actual int, t *testing.T) {
	if expected != actual {
		t.Error(fmt.Sprintf("Expected %d, was %d", expected, actual))
	}
}

func AssertStrEquals(expected, actual string, t *testing.T) {
	if expected != actual {
		t.Error(fmt.Sprintf("Expected %s, was %s", expected, actual))
	}
}
