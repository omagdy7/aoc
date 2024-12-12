package main

import (
	"fmt"
	"os"
	"strings"
)

type Direction string

const (
	Up          Direction = "up"
	Down        Direction = "down"
	Left        Direction = "left"
	Right       Direction = "right"
	TopLeft     Direction = "topleft"
	TopRight    Direction = "topRight"
	BottomLeft  Direction = "botleft"
	BottomRight Direction = "botRight"
)

func FileRead(path string) string {
	file, err := os.ReadFile(path)
	if err != nil {
		fmt.Println("Couldn't Read file: ", err)
	}
	return string(file)

}

func parseInput(data string) []string {
	return strings.Split(data, "\n")
}

func isValid(x int, y int, n int, m int) bool {
	return x >= 0 && x < n && y >= 0 && y < m
}

func solve_part_one(data string) int {
	grid := parseInput(data)
	grid = grid[:len(grid)-1]
	n := len(grid)
	m := len(grid[0])
	dx := [8]int{1, -1, 0, 0, 1, 1, -1, -1}
	dy := [8]int{0, 0, 1, -1, 1, -1, 1, -1}
	var words []string
	for x := range len(grid) {
		for y := range len(grid[0]) {
			if grid[x][y] == byte('X') {
				for i := range 8 {
					word := "X"
					for j := 1; j <= 3; j++ {
						nx := x + dx[i]*j
						ny := y + dy[i]*j
						if isValid(nx, ny, n, m) {
							word += string(grid[nx][ny])
						}
						words = append(words, word)
					}
				}
			}
		}
	}
	cnt := 0
	for _, word := range words {
		if word == "XMAS" {
			cnt++

		}
	}
	return cnt
}

func iSXMAS(window []string) bool {
	patterns := [][]string{{"M.M",
		".A.",
		"S.S"}, {"M.S",
		".A.",
		"M.S"}, {"S.S",
		".A.",
		"M.M"}, {"S.M",
		".A.",
		"S.M"}}
	for _, pat := range patterns {
		flag := true
		for i := range len(window) {
			for j := range len(window[0]) {
				if pat[i][j] != '.' {
					flag = flag && (pat[i][j] == window[i][j])
				}
			}
		}
		if flag {
			return true
		}
	}
	return false
}

func solve_part_two(data string) int {
	grid := parseInput(data)
	grid = grid[:len(grid)-1]
	n := len(grid)
	m := len(grid[0])
	var windows [][]string
	for x := range n - 2 {
		for y := range m - 2 {
			var window []string
			for i := range 3 {
				window = append(window, grid[i+y][x:x+3])
			}
			windows = append(windows, window)
		}
	}
	cnt := 0
	// fmt.Println(windows)
	for _, window := range windows {
		if iSXMAS(window) {
			cnt += 1
		}
	}
	return cnt
}

func main() {
	test := FileRead("../input/day04.test")
	prod := FileRead("../input/day04.prod")

	fmt.Println("Part_1 test: ", solve_part_one(test))
	fmt.Println("Part_1 prod: ", solve_part_one(prod))
	fmt.Println("Part_2 test: ", solve_part_two(test))
	fmt.Println("Part_2 prod: ", solve_part_two(prod))
}
