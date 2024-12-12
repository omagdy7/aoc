package main

import (
	"fmt"
	"os"
	"sort"
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

func abs(x, y int) int {
	if x-y < 0 {
		return -(x - y)
	} else {
		return x - y
	}
}

func bfs(grid Grid, start Point, seen *map[Point]bool) (int, int) {
	perimeter := 1
	area := 0
	w, h := len(grid), len(grid[0])
	var queue []Point
	(*seen)[start] = true
	queue = append(queue, start)
	dx := []int{1, -1, 0, 0} // down, up, right, left
	dy := []int{0, 0, 1, -1}
	for len(queue) > 0 {
		curPoint := queue[0]
		curPlant := grid[curPoint[0]][curPoint[1]]
		queue = queue[1:]
		for i := 0; i < 4; i++ {
			nx := curPoint[0] + dx[i]
			ny := curPoint[1] + dy[i]
			newPoint := Point{nx, ny}
			if inBounds(nx, ny, w, h) && grid[nx][ny] == curPlant {
				if !(*seen)[newPoint] {
					perimeter++
					queue = append(queue, newPoint)
					(*seen)[newPoint] = true
				}
			} else {
				area++
			}
		}
	}
	return perimeter, area
}

func getDir(dx, dy int) string {
	if dx == 0 && dy == 1 {
		return "R"
	} else if dx == 0 && dy == -1 {
		return "L"
	} else if dx == -1 && dy == 0 {
		return "U"
	} else if dx == 1 && dy == 0 {
		return "D"
	}
	return "unreachable"
}

func merge(walls [][4]int, dir int) int {
	cnt := 0
	for i := 0; i < len(walls)-1; i++ {
		cur := walls[i]
		next := walls[i+1]
		if dir == 0 { // x direction
			if cur[0] == next[0] && abs(cur[1], next[1]) == 1 && cur[2] == next[2] && cur[3] == next[3] {
				cnt++
			}
		} else if dir == 1 { // y direction
			if cur[1] == next[1] && abs(cur[0], next[0]) == 1 && cur[2] == next[2] && cur[3] == next[3] {
				cnt++
			} else {
			}
		}
	}
	return cnt
}

func printWalls(walls [][4]int) {
	for _, wall := range walls {
		fmt.Printf("(%d, %d) - %s / ", wall[0], wall[1], getDir(wall[2], wall[3]))
	}
	fmt.Println()
}

func bfs2(grid Grid, start Point, seen *map[Point]bool) (int, int) {
	perimeter := 1
	area := 0
	w, h := len(grid), len(grid[0])
	var queue []Point
	var walls [][4]int
	pointsX := make(map[int]map[Point]struct{})
	pointsX[start[0]] = make(map[Point]struct{})
	pointsX[start[0]][start] = struct{}{}
	(*seen)[start] = true
	queue = append(queue, start)
	dx := []int{1, -1, 0, 0} // down, up, right, left
	dy := []int{0, 0, 1, -1}
	for len(queue) > 0 {
		curPoint := queue[0]
		curPlant := grid[curPoint[0]][curPoint[1]]
		queue = queue[1:]
		for i := 0; i < 4; i++ {
			nx := curPoint[0] + dx[i]
			ny := curPoint[1] + dy[i]
			newPoint := Point{nx, ny}
			if inBounds(nx, ny, w, h) && grid[nx][ny] == curPlant {
				if !(*seen)[newPoint] {
					perimeter++
					queue = append(queue, newPoint)
					(*seen)[newPoint] = true
				}
			} else {
				area++
				walls = append(walls, [4]int{curPoint[0], curPoint[1], dx[i], dy[i]})
				if _, exists := pointsX[newPoint[0]]; !exists {
					pointsX[newPoint[0]] = make(map[Point]struct{})
				}
				pointsX[newPoint[0]][newPoint] = struct{}{}
			}
		}
	}

	sort.Slice(walls, func(i, j int) bool {
		if walls[i][0] != walls[j][0] {
			return walls[i][0] < walls[j][0] // Sort by the first element
		}
		if walls[i][2] != walls[j][2] {
			return walls[i][2] < walls[j][2] // Sort by the third element
		}
		if walls[i][3] != walls[j][3] {
			return walls[i][3] < walls[j][3] // Sort by the third element
		}
		return walls[i][1] < walls[j][1] // Sort by the second element
	})

	area -= merge(walls, 0)

	sort.Slice(walls, func(i, j int) bool {
		if walls[i][1] != walls[j][1] {
			return walls[i][1] < walls[j][1] // Sort by the second element
		}
		if walls[i][2] != walls[j][2] {
			return walls[i][2] < walls[j][2] // If second is equal, sort by the third element
		}
		if walls[i][3] != walls[j][3] {
			return walls[i][3] < walls[j][3] // If second and third are equal, sort by the fourth element
		}
		return walls[i][0] < walls[j][0] // If second, third, and fourth are equal, sort by the first element
	})

	area -= merge(walls, 1)
	return perimeter, area
}

type Point = [2]int
type Points = [][2]int

func solve_part_one(data string) int {
	gardenMap := parseInput(data)
	seen := make(map[Point]bool)
	w, h := len(gardenMap), len(gardenMap[0])
	ans := 0
	for i := 0; i < w; i++ {
		for j := 0; j < h; j++ {
			start := Point{i, j}
			if !seen[start] {
				perimeter, area := bfs(gardenMap, start, &seen)
				ans += perimeter * area
			}
		}
	}
	return ans
}

func solve_part_two(data string) int {
	gardenMap := parseInput(data)
	seen := make(map[Point]bool)
	w, h := len(gardenMap), len(gardenMap[0])
	ans := 0
	for i := 0; i < w; i++ {
		for j := 0; j < h; j++ {
			start := Point{i, j}
			if !seen[start] {
				perimeter, area := bfs2(gardenMap, start, &seen)
				ans += perimeter * area
			}
		}
	}
	return ans
}

func main() {
	test := FileRead("../input/day12.test")
	prod := FileRead("../input/day12.prod")
	fmt.Println("Part_1 test: ", solve_part_one(test))
	fmt.Println("Part_1 prod: ", solve_part_one(prod))
	fmt.Println("Part_2 test: ", solve_part_two(test))
	fmt.Println("Part_2 prod: ", solve_part_two(prod))
}
