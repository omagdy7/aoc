package main

import (
	"fmt"
	"os"
	"slices"
	"strings"
)

type Orientation = int

const (
	Up    Orientation = 0
	Right Orientation = 1
	Down  Orientation = 2
	Left  Orientation = 3
)

func FileRead(path string) string {
	file, err := os.ReadFile(path)
	if err != nil {
		fmt.Println("Couldn't Read file: ", err)
	}
	return string(file)

}

func turn90(orientation Orientation) Orientation {
	orientations := [4]Orientation{Up, Right, Down, Left}
	return orientations[(orientation+1)%4]
}

type Grid = []string

func isValid(grid *Grid, x int, y int) bool {
	n, m := len(*grid), len((*grid)[0])
	return x >= 0 && x < n && y >= 0 && y < m

}

func replaceCharGrid(grid *Grid, x int, y int, ch rune) {
	str := (*grid)[x]
	runes := []rune(str)
	runes[y] = ch
	(*grid)[x] = string(runes)
}

func simulateTrail(grid *Grid, gPos [2]int) int {
	set := make(map[[2]int]struct{})
	set[gPos] = struct{}{}
	curOrientation := getOrientation((*grid)[gPos[0]][gPos[1]])
	dx := [4]int{-1, 0, 1, 0} // Up, Right, Down, Left
	dy := [4]int{0, 1, 0, -1}
	for {
		nx := gPos[0] + dx[curOrientation]
		ny := gPos[1] + dy[curOrientation]
		if isValid(grid, nx, ny) {
			curPoint := (*grid)[nx][ny]
			if !isObstacle(curPoint) {
				gPos = [2]int{nx, ny}
				replaceCharGrid(grid, nx, ny, 'X')
				set[gPos] = struct{}{}
			} else {
				curOrientation = turn90(curOrientation)
			}
		} else {
			break
		}
	}
	return len(set)
}

func parseInput(data string) Grid {
	var grid Grid
	for _, line := range strings.Split(data, "\n") {
		if len(line) > 0 {
			grid = append(grid, line)
		}
	}
	return grid
}

func getOrientation(ch byte) Orientation {
	if ch == '^' {
		return Up
	} else if ch == '>' {
		return Right
	} else if ch == '<' {
		return Left
	} else {
		return Down
	}
}

func isGaurd(ch byte) bool {
	return slices.Contains([]string{"^", ">", "<", "V"}, string(ch))
}

func isObstacle(ch byte) bool {
	return ch == '#'
}

func solve_part_one(data string) int {
	grid := parseInput(data)
	var guardPosition [2]int
	for i, row := range grid {
		for j, ch := range row {
			if isGaurd(byte(ch)) {
				guardPosition = [2]int{i, j}
			}
		}
	}
	trailCount := simulateTrail(&grid, guardPosition)
	return trailCount
}

func isStuck(grid *Grid, gPos [2]int) bool {
	// n, m := len(*grid), len((*grid)[0])
	var visited [130][130][4]bool
	// visited := make([][][]bool, 4)
	// for i := range visited {
	// 	visited[i] = make([][]bool, n)
	// 	for j := range visited[i] {
	// 		visited[i][j] = make([]bool, m)
	// 	}
	// }
	curOrientation := getOrientation((*grid)[gPos[0]][gPos[1]])
	visited[gPos[0]][gPos[1]][curOrientation] = true
	dx := [4]int{-1, 0, 1, 0} // Up, Right, Down, Left
	dy := [4]int{0, 1, 0, -1}
	for {
		nx := gPos[0] + dx[curOrientation]
		ny := gPos[1] + dy[curOrientation]
		if isValid(grid, nx, ny) {
			curPoint := (*grid)[nx][ny]
			if !isObstacle(curPoint) {
				gPos = [2]int{nx, ny}
				if visited[nx][ny][curOrientation] {
					return true
				}
				visited[nx][ny][curOrientation] = true
			} else {
				curOrientation = turn90(curOrientation)
			}
		} else {
			break
		}
	}
	return false
}

func solve_part_two(data string) int {
	grid := parseInput(data)
	var guardPosition [2]int
	for i, row := range grid {
		for j, ch := range row {
			if isGaurd(byte(ch)) {
				guardPosition = [2]int{i, j}
			}
		}
	}
	n, m := len(grid), len(grid[0])
	ans := 0
	for i := range n {
		for j := range m {
			if !(i == guardPosition[0] && j == guardPosition[1]) {
				flag := grid[i][j] == '.'
				replaceCharGrid(&grid, i, j, '#')
				if isStuck(&grid, guardPosition) {
					ans++
				}
				if flag {
					replaceCharGrid(&grid, i, j, '.')
				}
			}
		}
	}
	return ans
}

func main() {
	test := FileRead("../input/day06.test")
	prod := FileRead("../input/day06.prod")
	fmt.Println("Part_1 test: ", solve_part_one(test))
	fmt.Println("Part_1 prod: ", solve_part_one(prod))
	fmt.Println("Part_2 test: ", solve_part_two(test))
	fmt.Println("Part_2 prod: ", solve_part_two(prod))
}
