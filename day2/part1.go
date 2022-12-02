package main

import (
	"errors"
	"fmt"
	"log"
	"strings"

	"github.com/mcasper/advent-of-code-2022/utils"
)

type HandSign int

const (
	Rock HandSign = iota
	Paper
	Scissors
	Unknown
)

func HandSignFromMove(move string) HandSign {
	switch move {
	case "A", "X":
		return Rock
	case "B", "Y":
		return Paper
	case "C", "Z":
		return Scissors
	}

	return Unknown
}

func SignScore(move HandSign) int64 {
	switch move {
	case Rock:
		return 1
	case Paper:
		return 2
	case Scissors:
		return 3
	}

	return 0
}

func WinScore(opponentMove, selfMove HandSign) int64 {
	if opponentMove == selfMove {
		return 3 // draw
	}

	if opponentMove == Rock && selfMove == Paper {
		return 6 // win
	}

	if opponentMove == Paper && selfMove == Scissors {
		return 6 // win
	}

	if opponentMove == Scissors && selfMove == Rock {
		return 6
	}

	return 0
}

func main() {
	lines, err := utils.FileLines("day2/input.txt")
	if err != nil {
		log.Fatal(err)
	}
	result, err := Solve(lines)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Day 2, Part 1 result: %d\n", result)
}

func Solve(lines []string) (int64, error) {
	score := int64(0)

	for _, line := range lines {
		moves := strings.Split(line, " ")
		if len(moves) != 2 {
			return score, errors.New("wrong number of moves on line")
		}

		opponentMove := HandSignFromMove(moves[0])
		selfMove := HandSignFromMove(moves[1])

		score += WinScore(opponentMove, selfMove)
		score += SignScore(selfMove)
	}

	return score, nil
}
