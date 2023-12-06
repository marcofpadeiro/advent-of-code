package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
	"time"
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

	var lines [][]string
	for scanner.Scan() {
		lines = append(lines, strings.Fields(scanner.Text()))
	}

	for i := 1; i < len(lines[0]); i++ {
		time, _ := strconv.Atoi(lines[0][i])
		distance, _ := strconv.Atoi(lines[1][i])

		b := float64(time)
		c := float64(distance)

		sqrt_1 := (-b - math.Sqrt(math.Pow(b, 2)-4*c)) / -2
		sqrt_2 := (-b + math.Sqrt(math.Pow(b, 2)-4*c)) / -2

		leftLimit := int(math.Floor(math.Min(sqrt_1, sqrt_2)))
		rightLimit := int(math.Ceil(math.Max(sqrt_1, sqrt_2)))

		number := rightLimit - leftLimit - 1

		total *= number
	}

	time := time.Since(start)

	fmt.Println("Result:", total)

	fmt.Println("Took", time)
}
