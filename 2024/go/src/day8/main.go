package main

import (
	"fmt"
	"os"
	"strings"
)

func FileRead(path string) string {
	file, err := os.ReadFile(path)
	if err != nil {
		fmt.Println("Couldn't Read file: ", err)
	}
	return string(file)

}

type Grid = [][]rune

func printGrid(grid [][]rune) {
	for _, row := range grid {
		for _, cell := range row {
			fmt.Print(string(cell))
		}
		fmt.Println()
	}
}

func parseInput(data string) Grid {
	lines := strings.Split(data, "\n")

	var grid Grid

	for _, line := range lines {
		if len(line) > 0 {
			runes := []rune(line)
			grid = append(grid, runes)
		}
	}
	return grid
}

func inBounds(x, y, n, m int) bool {
	return x >= 0 && x < n && y >= 0 && y < m
}

type Point = [2]int
type Points = [][2]int

func getPosMap(grid *Grid) map[rune]Points {
	n, m := len(*grid), len((*grid)[0])
	posMap := make(map[rune]Points)
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			curPos := Point{i, j}
			curRune := (*grid)[i][j]
			if curRune != '.' {
				if _, exists := posMap[curRune]; exists {
					posMap[curRune] = append(posMap[curRune], curPos)
				} else {
					posMap[curRune] = [][2]int{curPos}
				}
			}
		}
	}
	return posMap
}

func isValidAntenna(grid *Grid, i, j int) bool {
	n, m := len(*grid), len((*grid)[0])
	if inBounds(i, j, n, m) {
		return true
	}
	return false
}

func solve_part_one(data string) int {
	grid := parseInput(data)
	posMap := getPosMap(&grid)
	set := make(map[Point]struct{})
	for _, val := range posMap {
		for i := 0; i < len(val); i++ {
			curPosI := val[i]
			for j := i + 1; j < len(val); j++ {
				curPosJ := val[j]
				diffOne := [2]int{curPosI[0] - curPosJ[0], curPosI[1] - curPosJ[1]}
				diffTwo := [2]int{curPosJ[0] - curPosI[0], curPosJ[1] - curPosI[1]}
				firstAntennaPos := [2]int{curPosI[0] + diffOne[0], curPosI[1] + diffOne[1]}
				secondAntennaPos := [2]int{curPosJ[0] + diffTwo[0], curPosJ[1] + diffTwo[1]}
				if isValidAntenna(&grid, firstAntennaPos[0], firstAntennaPos[1]) {
					set[firstAntennaPos] = struct{}{}
				}
				if isValidAntenna(&grid, secondAntennaPos[0], secondAntennaPos[1]) {
					set[secondAntennaPos] = struct{}{}
				}
			}
		}
	}
	return len(set)
}

func solve_part_two(data string) int {
	grid := parseInput(data)
	posMap := getPosMap(&grid)
	cnt := 0
	for _, val := range posMap {
		for i := 0; i < len(val); i++ {
			curPosI := val[i]
			for j := i + 1; j < len(val); j++ {
				curPosJ := val[j]
				diffOne := [2]int{curPosI[0] - curPosJ[0], curPosI[1] - curPosJ[1]}
				diffTwo := [2]int{curPosJ[0] - curPosI[0], curPosJ[1] - curPosI[1]}
				firstAntennaPos := [2]int{curPosI[0] + diffOne[0], curPosI[1] + diffOne[1]}
				secondAntennaPos := [2]int{curPosJ[0] + diffTwo[0], curPosJ[1] + diffTwo[1]}
				for isValidAntenna(&grid, firstAntennaPos[0], firstAntennaPos[1]) {
					grid[firstAntennaPos[0]][firstAntennaPos[1]] = '#'
					firstAntennaPos = [2]int{firstAntennaPos[0] + diffOne[0], firstAntennaPos[1] + diffOne[1]}
				}
				for isValidAntenna(&grid, secondAntennaPos[0], secondAntennaPos[1]) {
					grid[secondAntennaPos[0]][secondAntennaPos[1]] = '#'
					secondAntennaPos = [2]int{secondAntennaPos[0] + diffTwo[0], secondAntennaPos[1] + diffTwo[1]}
				}
			}
		}
	}
	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid); j++ {
			if grid[i][j] != '.' {
				cnt++
			}
		}
	}
	return cnt
}

func main() {
	test := FileRead("../input/day8.test")
	prod := FileRead("../input/day8.prod")
	fmt.Println("Part_1 test: ", solve_part_one(test))
	fmt.Println("Part_1 prod: ", solve_part_one(prod))
	fmt.Println("Part_2 test: ", solve_part_two(test))
	fmt.Println("Part_2 prod: ", solve_part_two(prod))
}
