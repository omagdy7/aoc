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

type IVec2 struct {
	x int
	y int
}

type Robot struct {
	Pos      IVec2
	Velocity IVec2
}

func parseInput(data string) []Robot {
	var robots []Robot
	for _, line := range strings.Split(data, "\n") {
		var pos IVec2
		var vel IVec2
		if len(line) > 0 {
			splitted_line := strings.Split(line, " ")
			position := strings.Split(splitted_line[0][2:], ",")
			velcocity := strings.Split(splitted_line[1][2:], ",")
			pos.x, _ = strconv.Atoi(position[1])
			pos.y, _ = strconv.Atoi(position[0])
			vel.x, _ = strconv.Atoi(velcocity[1])
			vel.y, _ = strconv.Atoi(velcocity[0])
			robots = append(robots, Robot{Pos: pos, Velocity: vel})
		}
	}

	return robots
}

type Grid = [][]int

func mod(a, b int) int {
	r := a % b
	if r < 0 {
		r += b
	}
	return r
}

func printGrid(grid [][]int) {
	for _, row := range grid {
		for _, cell := range row {
			if cell == 0 {
				fmt.Printf(" ")
			} else {
				fmt.Printf("#")
			}
		}
		fmt.Println()
	}
}

func plotMap(robots []Robot, height, width int) Grid {
	grid := make([][]int, width)
	for i := range grid {
		grid[i] = make([]int, height)
	}
	for i := 0; i < width; i++ {
		for j := 0; j < height; j++ {
			grid[i][j] = 0
		}
	}
	for _, robot := range robots {
		grid[robot.Pos.x][robot.Pos.y] += 1
	}
	return grid
}

func plotNewMap(grid Grid, robots []Robot, height, width, seconds int) Grid {
	for _, robot := range robots {
		grid[robot.Pos.x][robot.Pos.y] -= 1
		newPos := IVec2{x: mod(robot.Pos.x+robot.Velocity.x*seconds, width),
			y: mod(robot.Pos.y+robot.Velocity.y*seconds, height)}
		grid[newPos.x][newPos.y] += 1
	}
	return grid
}

func countQuadrants(grid Grid) int64 {
	w, h := len(grid), len(grid[0])
	quadrants := [4]int{0, 0, 0, 0}

	for i := 0; i < w; i++ {
		if i == w/2 {
			continue
		}
		for j := 0; j < h; j++ {
			if j == h/2 {
				continue
			}
			row := 0
			if i >= w/2 {
				row = 1
			}
			col := 0
			if j >= h/2 {
				col = 1
			}
			quadrantIndex := row*2 + col
			quadrants[quadrantIndex] += grid[i][j]
		}
	}
	var ans int64 = 1
	for _, cnt := range quadrants {
		ans *= int64(cnt)
	}
	return ans

}

func solve_part_one(data string) int64 {
	robots := parseInput(data)
	// oldMap := plotMap(robots, 11, 7)
	// newMap := plotNewMap(oldMap, robots, 11, 7, 100)
	oldMap := plotMap(robots, 101, 103)
	newMap := plotNewMap(oldMap, robots, 101, 103, 100)
	ans := countQuadrants(newMap)
	return ans
}

func containsOverLaps(plotMap Grid) bool {
	for i := 0; i < len(plotMap); i++ {
		for j := 0; j < len(plotMap[0]); j++ {
			if plotMap[i][j] > 1 {
				return true
			}
		}
	}
	return false
}

func solve_part_two(data string) {
	robots := parseInput(data)
	oldMap := plotMap(robots, 101, 103)
	for steps := 1; steps <= 15000; steps++ {
		copiedMap := make([][]int, len(oldMap))
		for j := range oldMap {
			copiedMap[j] = make([]int, len(oldMap[j]))
			copy(copiedMap[j], oldMap[j])
		}
		newMap := plotNewMap(copiedMap, robots, 101, 103, steps)
		if !containsOverLaps(newMap) {
			printGrid(newMap)
			fmt.Println(steps)
		}
	}
}
func main() {
	// test := FileRead("../input/day14.test")
	prod := FileRead("../input/day14.prod")
	// fmt.Println("Part_1 test: ", solve_part_one(test))
	// fmt.Println("Part_1 prod: ", solve_part_one(prod))
	// fmt.Println("Part_2 test: ", solve_part_two(test))
	solve_part_two(prod)
}
