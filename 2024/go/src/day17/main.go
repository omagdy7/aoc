package main

import (
	"container/heap"
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

type Grid = [][]byte

type Point struct {
	x int
	y int
}

func parseInput(data string) Grid {
	lines := strings.Split(data, "\n")

	var grid Grid
	for _, line := range lines {
		if len(line) > 0 {
			bytes := []byte(line)
			grid = append(grid, bytes)
		}
	}
	return grid
}

func printGrid(grid [][]byte) {
	for _, row := range grid {
		for _, cell := range row {
			fmt.Printf(string(cell))
		}
		fmt.Println()
	}
}

func copySlice(src []Point) []Point {
	dest := make([]Point, len(src))
	copy(dest, src)
	return dest
}

func findMin(arr []int) int {
	min := arr[0]
	for _, v := range arr[1:] {
		if v < min {
			min = v
		}
	}
	return min
}

type Direction struct {
	dx, dy int
}

type State struct {
	cost      int
	stepCount int
	pos       Point
	dir       Direction
}

type PriorityQueue []*State

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].cost < pq[j].cost
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x interface{}) {
	item := x.(*State)
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[0 : n-1]
	return item
}

func inBounds(x, y, rows, cols int, grid [][]byte) bool {
	return x >= 0 && x < rows && y >= 0 && y < cols && grid[x][y] != '#'
}

func findPosition(grid [][]byte, target byte) (Point, bool) {
	for i := range grid {
		for j := range grid[i] {
			if grid[i][j] == target {
				return Point{i, j}, true
			}
		}
	}
	return Point{}, false
}

func dijkstra(grid [][]byte, start Point, end Point) (int, int, []Point) { // Returns cost, step count, and path
	if start == end {
		return 0, 0, []Point{start}
	}
	var directions = [4]Direction{
		{1, 0},  // Down
		{0, 1},  // Right
		{-1, 0}, // Up
		{0, -1}, // Left
	}
	pq := &PriorityQueue{}
	heap.Init(pq)
	type Key struct {
		x, y, dx, dy int
	}
	minCost := make(map[Key]int)
	stepCounts := make(map[Key]int)
	prev := make(map[Key]Key)          // Track previous position for path reconstruction
	prevDir := make(map[Key]Direction) // Track previous direction

	heap.Push(pq, &State{cost: 0, stepCount: 0, pos: start, dir: Direction{0, 0}})

	for pq.Len() > 0 {
		current := heap.Pop(pq).(*State)
		cost := current.cost
		stepCount := current.stepCount
		pos := current.pos
		dir := current.dir
		key := Key{pos.x, pos.y, dir.dx, dir.dy}

		if existingCost, exists := minCost[key]; exists && cost >= existingCost {
			continue
		}

		minCost[key] = cost
		stepCounts[key] = stepCount

		if pos == end {
			// Reconstruct path
			path := []Point{end}
			currentKey := key

			for currentKey.x != start.x || currentKey.y != start.y {
				previousKey := prev[currentKey]
				path = append([]Point{{previousKey.x, previousKey.y}}, path...)
				currentKey = previousKey
			}

			return cost, stepCount, path
		}

		for _, d := range directions {
			nx, ny := pos.x+d.dx, pos.y+d.dy
			if inBounds(nx, ny, len(grid), len(grid[0]), grid) {
				var newCost int
				if dir == (Direction{0, 0}) || d == dir {
					newCost = cost + 1
				} else {
					newCost = cost + 1001
				}
				newStepCount := stepCount + 1
				newKey := Key{nx, ny, d.dx, d.dy}

				if existingCost, exists := minCost[newKey]; !exists || newCost < existingCost {
					heap.Push(pq, &State{cost: newCost, stepCount: newStepCount, pos: Point{nx, ny}, dir: d})
					prev[newKey] = key    // Store the previous position
					prevDir[newKey] = dir // Store the previous direction
				}
			}
		}
	}

	return 0, 0, nil // 'S' is not reachable
}

func solve_part_one(data string) int {
	maze := parseInput(data)
	start, _ := findPosition(maze, 'E')
	end, _ := findPosition(maze, 'S')
	cost, _, _ := dijkstra(maze, start, end)
	return cost
}

func solve_part_two(data string) int {
	maze := parseInput(data)
	start, _ := findPosition(maze, 'E')
	end, _ := findPosition(maze, 'S')
	bestCost, _, path := dijkstra(maze, start, end)
	uniquePoints := make(map[Point]struct{})
	for _, p := range path {
		uniquePoints[p] = struct{}{}
	}

	fmt.Println(bestCost)

	for i := range maze {
		for j := range maze[i] {
			currentPoint := Point{i, j}
			_, exists := uniquePoints[currentPoint]
			if exists {
				continue
			}
			costCurToStart, _, pathStart := dijkstra(maze, start, currentPoint)
			costCurToEnd, _, pathEnd := dijkstra(maze, currentPoint, end)
			if costCurToStart+costCurToEnd+1000 == bestCost || costCurToStart+costCurToEnd == bestCost {
				for _, p := range pathStart {
					uniquePoints[p] = struct{}{}
				}
				for _, p := range pathEnd {
					uniquePoints[p] = struct{}{}
				}
				// fmt.Println(costCurToStart, costCurToEnd)
				uniquePoints[currentPoint] = struct{}{}
				maze[currentPoint.x][currentPoint.y] = 'O'
			}
		}
	}
	// printGrid(maze)
	return len(uniquePoints)
}

func main() {
	test := FileRead("../input/day16_1.test")
	// test := FileRead("../input/day16_2.test")
	prod := FileRead("../input/day16.prod")
	fmt.Println("Part_1 test: ", solve_part_one(test))
	fmt.Println("Part_1 prod: ", solve_part_one(prod))
	fmt.Println("Part_2 test: ", solve_part_two(test))
	fmt.Println("Part_2 prod: ", solve_part_two(prod))
}
