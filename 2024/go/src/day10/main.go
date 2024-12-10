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

type Point struct {
	x int
	y int
}

func (p Point) inBounds(w, h int) bool {
	return p.x >= 0 && p.x < w && p.y >= 0 && p.y < h
}

type Grid = [][]byte

func bfs(grid Grid, start Point, uniq bool) int {
	cnt := 0
	w, h := len(grid), len(grid[0])
	var queue []Point
	visited := make(map[Point]bool)
	visited[start] = true
	queue = append(queue, start)
	dx := []int{1, -1, 0, 0} // down, up, right, left
	dy := []int{0, 0, 1, -1}
	for len(queue) > 0 {
		curPoint := queue[0]
		queue = queue[1:] // pop from front
		for i := 0; i < 4; i++ {
			nx := curPoint.x + dx[i]
			ny := curPoint.y + dy[i]
			newPoint := Point{x: nx, y: ny}
			if uniq {
				if newPoint.inBounds(w, h) && !visited[newPoint] && int(grid[newPoint.x][newPoint.y]) == int(grid[curPoint.x][curPoint.y])+1 {
					visited[newPoint] = true
					queue = append(queue, newPoint)
					if grid[newPoint.x][newPoint.y] == byte('9') {
						cnt++
					}
				}
			} else {
				if newPoint.inBounds(w, h) && int(grid[newPoint.x][newPoint.y]) == int(grid[curPoint.x][curPoint.y])+1 {
					queue = append(queue, newPoint)
					if grid[newPoint.x][newPoint.y] == byte('9') {
						cnt++
					}
				}

			}
		}
	}
	return cnt
}

// String returns a string representation of the grid
func printGrid(g Grid) {
	for _, row := range g {
		for _, r := range row {
			fmt.Printf("%v ", string(r))
		}
		fmt.Println()
	}
}

func parseInput(data string) Grid {
	var grid Grid
	for _, line := range strings.Split(data, "\n") {
		if len(line) > 0 {
			row := []byte(line)
			grid = append(grid, row)
		}
	}
	return grid
}

func solve_part_one(data string) int {
	grid := parseInput(data)
	ans := 0
	for i, row := range grid {
		for j, byte := range row {
			if byte == '0' {
				ans += bfs(grid, Point{i, j}, true)
			}
		}
	}
	return ans
}

func solve_part_two(data string) int {
	grid := parseInput(data)
	ans := 0
	for i, row := range grid {
		for j, byte := range row {
			if byte == '0' {
				ans += bfs(grid, Point{i, j}, false)
			}
		}
	}
	return ans
}

func main() {
	test := FileRead("../input/day10.test")
	prod := FileRead("../input/day10.prod")
	fmt.Println("Part_1 test: ", solve_part_one(test))
	fmt.Println("Part_1 prod: ", solve_part_one(prod))
	fmt.Println("Part_2 test: ", solve_part_two(test))
	fmt.Println("Part_2 prod: ", solve_part_two(prod))
}
