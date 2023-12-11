package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

func main() {
	file, _ := os.Open("input.txt")

	defer file.Close()

	scanner := bufio.NewScanner(file)

	values := parse_input(scanner)

	start := time.Now()

	total := 0

	for _, line := range values {
		total += find_extrapolated_value(line, line[len(line)-1])
	}

	time_took := time.Since(start)
	fmt.Println("Result:", total)

	fmt.Println("Took", time_took)
}

func parse_input(scanner *bufio.Scanner) [][]int {
	var vals [][]int

	for scanner.Scan() {
		temp_str_arr := strings.Fields(scanner.Text())
		var temp_int_arr []int
		for _, c := range temp_str_arr {
			temp_int, _ := strconv.Atoi(c)
			temp_int_arr = append(temp_int_arr, temp_int)
		}
		vals = append(vals, temp_int_arr)
	}
	return vals
}

func find_extrapolated_value(line []int, total int) int {
	isValid := true
	for _, value := range line {
		if value != 0 {
			isValid = false
			break
		}
	}

	if isValid {
		return total
	}

	var new_line []int

	for i := 0; i < len(line)-1; i++ {
		new_line = append(new_line, line[i+1]-line[i])
	}

	return find_extrapolated_value(new_line, total+new_line[len(new_line)-1])

}
