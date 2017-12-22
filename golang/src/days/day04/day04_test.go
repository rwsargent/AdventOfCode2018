package main

import "testing"

func TestIsValid(t *testing.T) {
	if !isValid("aa bb cc dd ee") {
		t.Error("Should've worked!")
	}

	if isValid("aa bb cc dd aa") {
		t.Error("Case two incorrect")
	}

	if !isValid("aa bb cc dd aaa") {
		t.Error("Case three incorrect")
	}
}
