package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
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

	total := 1

	var lines []string

	for scanner.Scan() {
		temp := strings.Split(scanner.Text(), ": ")

		lines = append(lines, removeWhitespaces(temp[1]))
	}

	timeAvaliable, _ := strconv.Atoi(lines[0])
	distance, _ := strconv.Atoi(lines[1])

	b := float64(timeAvaliable)
	c := float64(distance)

	sqrt_1 := (-b - math.Sqrt(math.Pow(b, 2)-4*c)) / -2
	sqrt_2 := (-b + math.Sqrt(math.Pow(b, 2)-4*c)) / -2

	leftLimit := int(math.Floor(math.Min(sqrt_1, sqrt_2)))
	rightLimit := int(math.Ceil(math.Max(sqrt_1, sqrt_2)))

	number := rightLimit - leftLimit - 1

	total *= number

	time_took := time.Since(start)

	fmt.Println("Result:", total)

	fmt.Println("Took", time_took)
}

func removeWhitespaces(str string) string {
	var temp []rune
	for _, c := range str {
		if !unicode.IsSpace(c) {
			temp = append(temp, c)
		}
	}
	return string(temp)
}
