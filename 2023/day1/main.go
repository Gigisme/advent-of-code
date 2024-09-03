package main

import (
	"2023/utils"
	"fmt"
	"io"
	"regexp"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	lr, err := utils.NewLineReader("./day1/input.txt")
	if err != nil {
		fmt.Println("Error creating LineReader:", err)
		return
	}
	defer func(lr *utils.LineReader) {
		err := lr.Close()
		if err != nil {
			fmt.Println("Error closing LineReader:", err)
		}
	}(lr)

	var sumPart1, sumPart2 int

	for {
		line, err := lr.NextLine()
		if err == io.EOF {
			break
		}
		if err != nil {
			fmt.Println("Error reading line:", err)
		}

		sumPart1 += part1(line)
		sumPart2 += part2(line)

	}

	fmt.Println("Part 1:", sumPart1)
	fmt.Println("Part 2:", sumPart2)
}

func part1(line string) int {
	var left, right rune

	for _, r := range line {
		if unicode.IsDigit(r) {
			left = r
			break
		}
	}

	for i := len(line) - 1; i >= 0; i-- {
		if unicode.IsDigit(rune(line[i])) {
			//fmt.Println(line, rune(line[i]))
			right = rune(line[i])
			break
		}
	}

	firstDigit, _ := strconv.Atoi(string(left))
	lastDigit, _ := strconv.Atoi(string(right))

	number := firstDigit*10 + lastDigit
	return number
}

func part2(line string) int {
	var left, right int
	re := regexp.MustCompile(`\d|one|two|three|four|five|six|seven|eight|nine`)

	firstMatch := re.FindString(line)

	var lastMatch, substr string
	for i := len(line) - 1; i >= 0; i-- {
		substr = string(line[i]) + substr
		lastMatch = re.FindString(substr)
		if lastMatch != "" {
			break
		}
	}

	left, _ = numberToInt(firstMatch)
	right, _ = numberToInt(lastMatch)

	number := left*10 + right
	return number
}

func numberToInt(num string) (int, error) {
	if num, err := strconv.Atoi(num); err == nil {
		return num, nil
	}

	wordToNum := map[string]int{
		"one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7, "eight": 8, "nine": 9,
	}

	if num, exists := wordToNum[strings.ToLower(num)]; exists {
		return num, nil
	}

	return 0, fmt.Errorf("Invalid number: %s", num)
}
