package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func FileRead(path string) string {
	file, err := os.ReadFile(path)
	if err != nil {
		fmt.Println("Couldn't Read file: ", err)
	}
	return string(file)

}

func solve_part_one(data string) int {
	pattern := `mul\((\d+),(\d+)\)`
	regex := regexp.MustCompile(pattern)

	// FindStringSubmatch returns the full match and all capture groups
	matches := regex.FindAllStringSubmatch(data, -1)

	ans := 0

	// Iterate over all matches
	for _, match := range matches {
		// fmt.Printf("Match %d:\n", i+1)
		// fmt.Println("  Full match:", match[0]) // Entire match
		left, _ := strconv.Atoi(match[1])
		right, _ := strconv.Atoi(match[2])
		ans += left * right
	}
	return ans
}

func solve_part_two(data string) int {
	pattern := `mul\((\d+),(\d+)\)|don't\(\)|do\(\)`
	regex := regexp.MustCompile(pattern)

	// FindStringSubmatch returns the full match and all capture groups
	matches := regex.FindAllStringSubmatch(data, -1)

	ans := 0
	enabled := true

	// Iterate over all matches
	for _, match := range matches {
		left, _ := strconv.Atoi(match[1])
		right, _ := strconv.Atoi(match[2])
		if match[0] == "don't()" {
			enabled = false
		} else if match[0] == "do()" {
			enabled = true
		}
		if enabled {
			ans += left * right
		}
	}
	return ans
}

func main() {
	test := FileRead("../input/day3.test")
	test_2 := FileRead("../input/day3_2.test")
	prod := FileRead("../input/day3.prod")
	// Define the regex pattern with capture groups

	fmt.Println("Part_1 test: ", solve_part_one(test))
	fmt.Println("Part_1 prod: ", solve_part_one(prod))
	fmt.Println("Part_2 test: ", solve_part_two(test_2))
	fmt.Println("Part_2 prod: ", solve_part_two(prod))
}
