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

type Outcome int

const (
	Lose Outcome = iota
	Draw
	Win
	OutcomeUnknown
)

func HandSignFromMove(move string) HandSign {
	switch move {
	case "A":
		return Rock
	case "B":
		return Paper
	case "C":
		return Scissors
	}

	return Unknown
}

func OutcomeFromMove(move string) Outcome {
	switch move {
	case "X":
		return Lose
	case "Y":
		return Draw
	case "Z":
		return Win
	}

	return OutcomeUnknown
}

func HandSignFromOutcome(opponentMove HandSign, outcome Outcome) HandSign {
	switch outcome {
	case Lose:
		if opponentMove == Rock {
			return Scissors
		}

		if opponentMove == Paper {
			return Rock
		}

		if opponentMove == Scissors {
			return Paper
		}
	case Draw:
		return opponentMove
	case Win:
		if opponentMove == Rock {
			return Paper
		}

		if opponentMove == Paper {
			return Scissors
		}

		if opponentMove == Scissors {
			return Rock
		}
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
	result, err := SolvePart2(lines)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Day 2, Part 2 result: %d\n", result)
}

func SolvePart2(lines []string) (int64, error) {
	score := int64(0)

	for _, line := range lines {
		moves := strings.Split(line, " ")
		if len(moves) != 2 {
			return score, errors.New("wrong number of moves on line")
		}

		opponentMove := HandSignFromMove(moves[0])
		outcome := OutcomeFromMove(moves[1])
		selfMove := HandSignFromOutcome(opponentMove, outcome)

		score += WinScore(opponentMove, selfMove)
		score += SignScore(selfMove)
	}

	return score, nil
}
