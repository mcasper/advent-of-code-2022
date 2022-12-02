package main

import (
	"testing"

	"github.com/mcasper/advent-of-code-2022/utils"
)

func TestSolve(t *testing.T) {
	expected := int64(15)
	lines, err := utils.TestLines()
	if err != nil {
		t.Error(err)
		return
	}

	result, err := Solve(lines)
	if err != nil {
		t.Error(err)
		return
	}

	if expected != result {
		t.Errorf("%d is not equal to %d", result, expected)
	}
}
