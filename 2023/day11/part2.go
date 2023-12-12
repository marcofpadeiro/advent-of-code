package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"time"
)

type Coordinates struct {
	x int
	y int
}

func main() {

	file, err := os.Open("input.txt")

	if err != nil {
		fmt.Println(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	start := time.Now()

	total := 0

	input := parse_input(scanner)
	empty_cols := get_empty_cols(input)
	empty_lines := get_empty_lines(input)
	galaxies := get_galaxies(input)

	for i, galaxy := range galaxies {
		for j, pair := range galaxies {
			if i > j {
				total += find_distance(galaxy, pair, empty_lines, empty_cols)
			}
		}
	}

	time_took := time.Since(start)
	fmt.Println("Result:", total)

	fmt.Println("Took", time_took)
}

func parse_input(scanner *bufio.Scanner) [][]rune {
	var lines [][]rune

	for scanner.Scan() {
		lines = append(lines, []rune(scanner.Text()))
	}

	return lines
}

func get_empty_cols(arr [][]rune) []int {
	var temp []int
	for i := 0; i < len(arr[0]); i++ {
		empty := true
		for j := 0; j < len(arr); j++ {
			if arr[j][i] != '.' {
				empty = false
				break
			}
		}
		if empty {
			temp = append(temp, i)
		}
	}
	return temp
}

func get_empty_lines(arr [][]rune) []int {
	var temp []int
	for i := 0; i < len(arr); i++ {
		empty := true
		for j := 0; j < len(arr[0]); j++ {
			if arr[i][j] != '.' {
				empty = false
				break
			}
		}
		if empty {
			temp = append(temp, i)
		}
	}
	return temp
}

func get_galaxies(arr [][]rune) []Coordinates {
	var temp []Coordinates
	for x, line := range arr {
		for y, c := range line {
			if c == '#' {
				temp = append(temp, Coordinates{x, y})
			}
		}
	}
	return temp
}

func find_distance(pair1, pair2 Coordinates, empty_cols, empty_lines []int) int {
	xMin := int(math.Min(float64(pair1.x), float64(pair2.x)))
	xMax := int(math.Max(float64(pair1.x), float64(pair2.x)))
	yMin := int(math.Min(float64(pair1.y), float64(pair2.y)))
	yMax := int(math.Max(float64(pair1.y), float64(pair2.y)))

	var emptyLinesBetween, emptyColsBetween int
	for _, c := range empty_cols {
		if c > xMin && c < xMax {
			emptyColsBetween++
		}
	}
	for _, c := range empty_lines {
		if c > yMin && c < yMax {
			emptyLinesBetween++
		}
	}

	distanceX := xMax - xMin - emptyColsBetween + (emptyColsBetween * 1000000)
	distanceY := yMax - yMin - emptyLinesBetween + (emptyLinesBetween * 1000000)

	return distanceX + distanceY
}
