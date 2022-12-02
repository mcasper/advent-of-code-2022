package main

import (
	"testing"

	"github.com/mcasper/advent-of-code-2022/utils"
)

func TestSolvePart2(t *testing.T) {
	expected := int64(12)
	lines, err := utils.TestLines()
	if err != nil {
		t.Error(err)
		return
	}

	result, err := SolvePart2(lines)
	if err != nil {
		t.Error(err)
		return
	}

	if expected != result {
		t.Errorf("%d is not equal to %d", result, expected)
	}
}
