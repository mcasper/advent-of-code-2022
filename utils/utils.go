package utils

import (
	"os"
	"strings"
)

func TestLines() ([]string, error) {
	return FileLines("test.txt")
}

func InputLines() ([]string, error) {
	return FileLines("input.txt")
}

func FileLines(filepath string) ([]string, error) {
	content, err := os.ReadFile(filepath)
	if err != nil {
		return []string{}, err
	}

	lines := strings.Split(string(content), "\n")
	result := []string{}

	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if trimmed != "" {
			result = append(result, line)
		}

	}

	return result, nil
}
