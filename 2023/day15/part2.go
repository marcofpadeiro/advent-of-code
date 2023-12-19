package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

type Lens struct {
	label  string
	number int
}

func main() {
	file, _ := os.Open("input.txt")

	defer file.Close()

	scanner := bufio.NewScanner(file)

	input := get_input(scanner)

	start := time.Now()

	total := 0

	var boxes [256][]Lens

	for _, block := range input {
		if strings.Contains(block, "=") {
			temp := strings.Split(block, "=")
			temp_num, _ := strconv.Atoi(temp[1])
			lens := Lens{temp[0], temp_num}

			box_num := hash_algo([]rune(lens.label))

			exists := false
			for current_lens, l := range boxes[box_num] {
				if lens.label == l.label {
					boxes[box_num][current_lens].number = lens.number
					exists = true
				}
			}
			if !exists {
				boxes[box_num] = append(boxes[box_num], lens)
			}
		} else {
			label := block[:len(block)-1]
			for current_box, box := range boxes {
				for current_lens, lens := range box {
					if lens.label == label {
						boxes[current_box] = append(boxes[current_box][:current_lens], boxes[current_box][current_lens+1:]...)
					}
				}
			}
		}
	}

	for i, box := range boxes {
		for j, block := range box {
			sum := i + 1
			sum *= (j + 1)
			sum *= block.number
			total += sum
		}
	}

	time_took := time.Since(start)
	fmt.Println("Result:", total)

	fmt.Println("Took", time_took)
}

func get_input(scanner *bufio.Scanner) []string {
	scanner.Scan()
	return strings.Split(scanner.Text(), ",")
}

func hash_algo(str []rune) int {
	sum := 0
	for _, c := range str {
		sum += int(c)
		sum *= 17
		sum %= 256
	}
	return sum
}
