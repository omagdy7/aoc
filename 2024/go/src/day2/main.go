package main

import (
	"fmt"
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

func parseInput(data string) [][]int {
	var reports [][]int
	for _, line := range strings.Split(data, "\n") {
		var report []int
		if len(line) > 0 {
			elements := strings.Split(line, " ")
			for _, ele := range elements {
				e, err := strconv.Atoi(ele)
				if err != nil {
					fmt.Println("Couldn't parse string to int ", err)
				}
				report = append(report, e)
			}
			reports = append(reports, report)
		}
	}
	return reports
}

func isSafe(report []int) bool {
	increasing := report[0] < report[1]
	flag := true
	for i := range len(report) - 1 {
		if increasing {
			diff := report[i+1] - report[i]
			if report[i] >= report[i+1] || diff > 3 || diff < 1 {
				flag = false
				break
			}
		} else {
			diff := report[i] - report[i+1]
			if report[i] <= report[i+1] || diff > 3 || diff < 1 {
				flag = false
				break
			}
		}
	}
	return flag
}

func solve_part_one(data string) int {
	reports := parseInput(data)
	cnt := 0
	for _, report := range reports {
		if isSafe(report) {
			cnt += 1
		}
	}
	return cnt
}

func solve_part_two(data string) int {
	reports := parseInput(data)
	cnt := 0
	for _, report := range reports {
		if isSafe(report) {
			cnt += 1
		} else {
			for i := range len(report) {
				reportTmp := make([]int, len(report))
				copy(reportTmp, report)
				newReport := append(reportTmp[:i], reportTmp[i+1:]...)
				if isSafe(newReport) {
					cnt += 1
					break
				}
			}
		}
	}
	return cnt
}

func main() {
	test := FileRead("../input/day_2.test")
	prod := FileRead("../input/day_2.prod")

	fmt.Println("Part_1 test: ", solve_part_one(test))
	fmt.Println("Part_1 prod: ", solve_part_one(prod))
	fmt.Println("Part_2 test: ", solve_part_two(test))
	fmt.Println("Part_2 prod: ", solve_part_two(prod))
}
