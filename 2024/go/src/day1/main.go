package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func AbsInt(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func solve_part_one(data string) int {
	var list_1 []int
	var list_2 []int

	ans := 0
	for _, line := range strings.Split(data, "\n") {
		if len(line) > 0 {
			elements := strings.Split(line, "   ")
			ele1, _ := strconv.Atoi(elements[0])
			ele2, _ := strconv.Atoi(elements[1])
			list_1 = append(list_1, ele1)
			list_2 = append(list_2, ele2)
		}
	}
	sort.Ints(list_1)
	sort.Ints(list_2)
	for i := range len(list_1) {
		ans += AbsInt(list_1[i] - list_2[i])
	}
	return ans
}

func solve_part_two(data string) int {
	var list_1 []int
	var list_2 []int

	ans := 0
	for _, line := range strings.Split(data, "\n") {
		if len(line) > 0 {
			elements := strings.Split(line, "   ")
			ele1, _ := strconv.Atoi(elements[0])
			ele2, _ := strconv.Atoi(elements[1])
			list_1 = append(list_1, ele1)
			list_2 = append(list_2, ele2)
		}
	}
	counts := make(map[int]int)

	for _, num := range list_2 {
		counts[num]++
	}
	for i := range len(list_1) {
		ans += list_1[i] * counts[list_1[i]]
	}
	return ans
}

func main() {
	test_1, _ := os.ReadFile("../input/day_1.test")
	test_2, _ := os.ReadFile("../input/day_1_2.test")
	prod, _ := os.ReadFile("../input/day_1.prod")
	content_test_1 := string(test_1)
	content_test_2 := string(test_2)
	content_prod := string(prod)

	fmt.Println("Part_1 test: ", solve_part_one(content_test_1))
	fmt.Println("Part_1 prod: ", solve_part_one(content_prod))
	fmt.Println("Part_2 test: ", solve_part_two(content_test_2))
	fmt.Println("Part_2 prod: ", solve_part_two(content_prod))
}
