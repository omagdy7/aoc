package main

import (
	"fmt"
	"os"
	"slices"
	"sort"
	"strconv"
	"strings"
)

type SafetyManual struct {
	pageOrderingRules [][2]int
	updates           [][]int
}

func (s SafetyManual) String() string {
	var builder strings.Builder

	// Format pageOrderingRules
	builder.WriteString("Page Ordering Rules: [")
	for i, rule := range s.pageOrderingRules {
		if i > 0 {
			builder.WriteString(", ")
		}
		builder.WriteString(fmt.Sprintf("[%d, %d]", rule[0], rule[1]))
	}
	builder.WriteString("]\n")

	// Format updates
	builder.WriteString("Updates: [")
	for i, update := range s.updates {
		if i > 0 {
			builder.WriteString(", ")
		}
		builder.WriteString("[")
		for j, val := range update {
			if j > 0 {
				builder.WriteString(", ")
			}
			builder.WriteString(fmt.Sprintf("%d", val))
		}
		builder.WriteString("]")
	}
	builder.WriteString("]")

	return builder.String()
}

func FileRead(path string) string {
	file, err := os.ReadFile(path)
	if err != nil {
		fmt.Println("Couldn't Read file: ", err)
	}
	return string(file)

}

func parseInput(data string) SafetyManual {
	section := strings.Split(data, "\n\n")
	first_section := strings.Split(section[0], "\n")
	second_section := strings.Split(section[1], "\n")
	var pageOrderingRules [][2]int
	var updates [][]int
	for _, rule := range first_section {
		pages := strings.Split(rule, "|")
		pageOne, _ := strconv.Atoi(pages[0])
		pageTwo, _ := strconv.Atoi(pages[1])
		arr := [2]int{pageOne, pageTwo}
		pageOrderingRules = append(pageOrderingRules, arr)
	}
	for _, line := range second_section {
		line := strings.Split(line, ",")
		var update []int
		for _, ele := range line {
			ele, _ := strconv.Atoi(ele)
			update = append(update, ele)
		}
		updates = append(updates, update)
	}
	return SafetyManual{
		pageOrderingRules: pageOrderingRules,
		updates:           updates,
	}

}

func filter_relevant_rules(s *SafetyManual, update *[]int) [][2]int {
	var filtered_rules [][2]int
	for _, rule := range s.pageOrderingRules {
		if slices.Contains(*update, rule[0]) && slices.Contains(*update, rule[1]) {
			filtered_rules = append(filtered_rules, rule)
		}
	}
	return filtered_rules
}

func makeProperList(items [][2]int) []int {
	counter := make(map[int]int)

	for _, item := range items {
		counter[item[0]]++
		if _, exists := counter[item[1]]; !exists {
			counter[item[1]] = 0
		}
	}

	var sorted [][2]int

	for val, fq := range counter {
		arr := [2]int{fq, val}
		sorted = append(sorted, arr)
	}

	sort.Slice(sorted, func(i, j int) bool {
		return sorted[i][0] > sorted[j][0]
	})

	var properList []int

	for _, x := range sorted {
		if len(x) == 0 {
			break
		}
		properList = append(properList, x[1])
	}

	return properList
}

func solve_part_one(data string) int {
	section := parseInput(data)
	ans := 0
	for _, update := range section.updates {
		filtered_rules := filter_relevant_rules(&section, &update)
		properList := makeProperList(filtered_rules)
		flag := true
		for i := range properList {
			if properList[i] != update[i] {
				flag = false
				break
			}
		}
		if flag {
			ans += update[len(update)/2]
		}
	}
	return ans
}

func solve_part_two(data string) int {
	section := parseInput(data)
	ans := 0
	for _, update := range section.updates {
		filtered_rules := filter_relevant_rules(&section, &update)
		properList := makeProperList(filtered_rules)
		flag := true
		for i := range properList {
			if properList[i] != update[i] {
				flag = false
				break
			}
		}
		if !flag {
			ans += properList[len(properList)/2]
		}
	}
	return ans
}

func main() {
	test := FileRead("../input/day5.test")
	prod := FileRead("../input/day5.prod")

	fmt.Println("Part_1 test: ", solve_part_one(test))
	fmt.Println("Part_1 prod: ", solve_part_one(prod))
	fmt.Println("Part_2 test: ", solve_part_two(test))
	fmt.Println("Part_2 prod: ", solve_part_two(prod))
}
