package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"time"
	"unicode"
)

func main() {
	file, err := os.Open("input.txt")

	if err != nil {
		fmt.Println(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	start := time.Now()

	total := 0

	var lines [][]rune

	for scanner.Scan() {
		lines = append(lines, []rune(scanner.Text()))
	}

	for line_index, line := range lines {
		for i := 0; i < len(line); i++ {
			if line[i] == 42 {
				gear1, gear2 := checkForGears(lines, line_index, i)

				if gear1 != 0 && gear2 != 0 {
					total += gear1 * gear2
				}

			}
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Print(err)
	}

	fmt.Println("Result:", total)

	fmt.Println("Took", time.Since(start))
}

func checkForGears(lines [][]rune, line_index, col_index int) (int, int) {
	topMargin, rightMargin, bottomMargin, leftMargin := line_index, col_index, line_index, col_index

	if line_index > 0 {
		topMargin = line_index - 1
	}
	if col_index < len(lines[line_index])-1 {
		rightMargin = col_index + 1
	}
	if line_index < len(lines)-1 {
		bottomMargin = line_index + 1
	}
	if col_index > 0 {
		leftMargin = col_index - 1
	}

	var gears [2]int = [2]int{0, 0}
	inNumber := false
	currGear := 0
	var temp string

	for i := topMargin; i <= bottomMargin; i++ {
		for j := leftMargin; j <= rightMargin; j++ {
			if i == line_index && j == col_index {
				continue
			}

			if unicode.IsDigit(lines[i][j]) {
				inNumber = true
				temp = extractNumber(string(lines[i]), j)
			}
			if inNumber && temp != "" {
				inNumber = false
				temp_int, _ := strconv.Atoi(temp)
				temp = ""
				if temp_int != gears[0] {
					gears[currGear] = temp_int
					currGear++
					if currGear == 2 {
						return gears[0], gears[1]
					}
				}
			}
		}
	}

	return gears[0], gears[1]
}

func extractNumber(s string, currIndex int) string {
	leftPointer := currIndex
	rightPointer := currIndex

	for leftPointer >= 0 && unicode.IsDigit(rune(s[leftPointer])) {
		leftPointer--
	}

	for rightPointer < len(s) && unicode.IsDigit(rune(s[rightPointer])) {
		rightPointer++
	}

	number := s[leftPointer+1 : rightPointer]

	return number
}
