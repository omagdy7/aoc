package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func FileRead(path string) string {
	file, err := os.ReadFile(path)
	if err != nil {
		fmt.Println("Couldn't Read file: ", err)
	}
	return string(file)
}

func parseInput(data string) []int {
	var stones []int
	for _, x := range strings.Split(data, " ") {
		x := strings.TrimRight(x, "\n")
		stone, _ := strconv.Atoi(x)
		stones = append(stones, stone)
	}
	return stones
}

func insert(slice []int, index int, value int) []int {
	if index < 0 || index > len(slice) {
		return slice // Or handle the error as you prefer
	}
	return append(slice[:index], append([]int{value}, slice[index:]...)...)
}

func getNumDigits(digit int) int {
	cnt := 0
	for digit != 0 {
		digit /= 10
		cnt++
	}
	return cnt
}

func splitNumber(num int) (int, int) {
	digits := getNumDigits(num)
	divisor := int(math.Pow10(digits / 2))
	left := num / divisor
	right := num % divisor
	return left, right
}

func isEvenDigits(digit int) bool {
	return getNumDigits(digit)%2 == 0
}

func processStone(cache *map[[2]int]int, stone, blinks int) int {
	result := 0
	if blinks == 0 {
		return 1
	} else {
		_, exists := (*cache)[[2]int{stone, blinks}]
		if !exists {
			if stone == 0 {
				result = processStone(cache, 1, blinks-1)
			} else if isEvenDigits(stone) {
				left, right := splitNumber(stone)
				result += processStone(cache, left, blinks-1)
				result += processStone(cache, right, blinks-1)
			} else {
				result = processStone(cache, stone*2024, blinks-1)
			}
			(*cache)[[2]int{stone, blinks}] = result
		}
		return (*cache)[[2]int{stone, blinks}]
	}
}
func processStones(stones []int, blinks int) int {
	cache := make(map[[2]int]int)
	ans := 0
	n := len(stones)
	for j := 0; j < n; j++ {
		ans += processStone(&cache, stones[j], blinks)
	}
	return ans
}

func solve_part_one(data string) int {
	stones := parseInput(data)
	ans := processStones(stones, 25)
	return ans
}

func solve_part_two(data string) int {
	stones := parseInput(data)
	ans := processStones(stones, 75)
	return ans
}

func main() {
	test := FileRead("../input/day11.test")
	prod := FileRead("../input/day11.prod")
	fmt.Println("Part_1 test: ", solve_part_one(test))
	fmt.Println("Part_1 prod: ", solve_part_one(prod))
	fmt.Println("Part_2 test: ", solve_part_two(test))
	fmt.Println("Part_2 prod: ", solve_part_two(prod))
}
