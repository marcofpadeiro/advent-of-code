package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

type Play struct {
	hand []rune
	bid  int
}

const (
	Five_of_kind  = 6
	Four_of_kind  = 5
	Full_house    = 4
	Three_of_kind = 3
	Two_pair      = 2
	One_pair      = 1
	High_card     = 0
)
const (
	A = 14
	K = 13
	Q = 12
	J = 11
	T = 10
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
	var plays []Play

	for scanner.Scan() {
		temp := strings.Fields(scanner.Text())
		hand := []rune(temp[0])
		bid, _ := strconv.Atoi(temp[1])

		plays = append(plays, Play{hand, bid})
	}

	mergeSort(plays, 0, len(plays)-1)

	for i, play := range plays {
		total += play.bid * (len(plays) - i)
	}

	time := time.Since(start)
	fmt.Println("Result:", total)

	fmt.Println("Took", time)
}

func playCmp(hand1, hand2 []rune) int {
	hand1_type := getHandType(hand1)
	hand2_type := getHandType(hand2)

	winner := 0

	if hand1_type > hand2_type {
		winner = 1
	} else if hand1_type < hand2_type {
		winner = -1
	} else { // equal types == tie breaker
		for i := 0; i < len(hand1); i++ {
			if runeToCard(hand1[i]) > runeToCard(hand2[i]) {
				winner = 1
				break
			} else if runeToCard(hand1[i]) < runeToCard(hand2[i]) {
				winner = -1
				break
			}
		}
	}
	return winner
}

func getHandType(hand []rune) int {
	handMap := make(map[rune]int)

	for _, c := range hand {
		handMap[c]++
	}

	handType := -1
	for _, value := range handMap {
		switch value {
		case 5:
			return Five_of_kind
		case 4:
			return Four_of_kind
		case 3:
			if handType == One_pair {
				return Full_house
			} else {
				handType = Three_of_kind
				break
			}
		case 2:
			if handType == One_pair {
				return Two_pair
			} else if handType == Three_of_kind {
				return Full_house
			} else {
				handType = One_pair
			}
		case 1:
			if handType == -1 {
				handType = High_card
			}
		}
	}
	return handType
}

func runeToCard(r rune) int {
	switch r {
	case 'A':
		return A
	case 'K':
		return K
	case 'Q':
		return Q
	case 'J':
		return J
	case 'T':
		return T
	default:
		return int(r - '0')
	}
}

func mergeSort(plays []Play, leftLimit, rightLimit int) {
	if leftLimit < rightLimit {
		middle := leftLimit + (rightLimit-leftLimit)/2

		mergeSort(plays, leftLimit, middle)
		mergeSort(plays, middle+1, rightLimit)

		merge(plays, leftLimit, middle, rightLimit)
	}
}

func merge(plays []Play, p, q, r int) {
	n1 := q - p + 1
	n2 := r - q

	L := make([]Play, n1)
	M := make([]Play, n2)

	for i := 0; i < n1; i++ {
		L[i] = plays[p+i]
	}

	for j := 0; j < n2; j++ {
		M[j] = plays[q+1+j]
	}

	i := 0
	j := 0
	k := p

	for i < n1 && j < n2 {
		if playCmp(L[i].hand, M[j].hand) == 1 {
			plays[k] = L[i]
			i++
		} else {
			plays[k] = M[j]
			j++
		}
		k++
	}

	for i < n1 {
		plays[k] = L[i]
		i++
		k++
	}
	for j < n2 {
		plays[k] = M[j]
		j++
		k++
	}
}

