package main

import (
	"fmt"
	"os"
	"slices"
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

type Input struct {
	warehouse  Grid
	directions []byte
}

func getRobotsPos(warehouse Grid) Point {
	for i := 0; i < len(warehouse); i++ {
		for j := 0; j < len(warehouse[0]); j++ {
			if warehouse[i][j] == '@' {
				return Point{i, j}
			}
		}
	}
	return Point{0, 0}
}

func parseInput(data string) Input {
	splitted_data := strings.Split(data, "\n\n")
	directions := []byte(splitted_data[1])
	var warehouse Grid
	for _, line := range strings.Split(splitted_data[0], "\n") {
		row := []byte(line)
		warehouse = append(warehouse, row)
	}
	return Input{
		warehouse:  warehouse,
		directions: directions,
	}
}

func printDirs(dirs []byte) {
	for _, dir := range dirs {
		fmt.Printf(string(dir))
	}
}

func printGrid(grid [][]byte) {
	for _, row := range grid {
		for _, cell := range row {
			fmt.Printf(string(cell))
		}
		fmt.Println()
	}
}

func inBounds(x, y, w, h int) bool {
	return x >= 0 && x < w && y >= 0 && y < h
}

func isBlock(x, y int, warehouse Grid) bool {
	return slices.Contains([]byte{'O', '[', ']'}, warehouse[x][y])
}

func isWall(x, y int, warehouse Grid) bool {
	return warehouse[x][y] == '#'
}

func constructWideWarehouse(warehouse Grid) Grid {
	var newWarehouse Grid
	for i := 0; i < len(warehouse); i++ {
		var row []byte
		for j := 0; j < len(warehouse[0]); j++ {
			if warehouse[i][j] == '.' {
				row = append(row, '.')
				row = append(row, '.')
			} else if warehouse[i][j] == 'O' {
				row = append(row, '[')
				row = append(row, ']')
			} else if warehouse[i][j] == '@' {
				row = append(row, '@')
				row = append(row, '.')
			} else {
				row = append(row, '#')
				row = append(row, '#')
			}
		}
		newWarehouse = append(newWarehouse, row)
	}
	return newWarehouse
}

func simulate(input Input) Grid {
	dirMap := map[byte]Point{
		'<': {0, -1},
		'>': {0, 1},
		'^': {-1, 0},
		'v': {1, 0},
	}
	w, h := len(input.warehouse), len(input.warehouse[0])
	robotPos := getRobotsPos(input.warehouse)
	for _, dir := range input.directions {
		dx, dy := dirMap[dir].x, dirMap[dir].y
		nx, ny := robotPos.x+dx, robotPos.y+dy
		// printGrid(input.warehouse)
		if inBounds(nx, ny, w, h) && !isWall(nx, ny, input.warehouse) {
			if input.warehouse[nx][ny] == '.' {
				input.warehouse[nx][ny], input.warehouse[robotPos.x][robotPos.y] = input.warehouse[robotPos.x][robotPos.y], input.warehouse[nx][ny]
				robotPos.x = nx
				robotPos.y = ny
			} else if isBlock(nx, ny, input.warehouse) {
				step := 0
				for isBlock(nx+dx*step, ny+dy*step, input.warehouse) {
					step++
				}
				if !isWall(nx+dx*step, ny+dy*step, input.warehouse) {
					for i := step; i >= 1; i-- {
						// swaps blocks backwares
						input.warehouse[nx+dx*(i-1)][ny+dy*(i-1)], input.warehouse[nx+dx*(i)][ny+dy*(i)] = input.warehouse[nx+dx*(i)][ny+dy*(i)], input.warehouse[nx+dx*(i-1)][ny+dy*(i-1)]
					}
					input.warehouse[nx][ny], input.warehouse[robotPos.x][robotPos.y] = input.warehouse[robotPos.x][robotPos.y], input.warehouse[nx][ny]
					robotPos.x = nx
					robotPos.y = ny
				}
			}
		}
	}
	return input.warehouse
}

func getBlocksToMove(pos Point, dir byte, blocksToMove *map[Point]bool, warehosue Grid) {
	dirMap := map[byte]Point{
		'<': {0, -1},
		'>': {0, 1},
		'^': {-1, 0},
		'v': {1, 0},
	}
	x, y := pos.x, pos.y
	(*blocksToMove)[pos] = true
	dx, dy := dirMap[dir].x, dirMap[dir].y
	if warehosue[x][y] == '[' {
		if !(*blocksToMove)[Point{x, y + 1}] {
			getBlocksToMove(Point{x, y + 1}, dir, blocksToMove, warehosue)
		}
		if isBlock(x+dx, y+dy, warehosue) {
			getBlocksToMove(Point{x + dx, y + dy}, dir, blocksToMove, warehosue)
		}
	} else if warehosue[x][y] == ']' {
		if !(*blocksToMove)[Point{x, y - 1}] {
			getBlocksToMove(Point{x, y - 1}, dir, blocksToMove, warehosue)
		}
		if isBlock(x+dx, y+dy, warehosue) {
			getBlocksToMove(Point{x + dx, y + dy}, dir, blocksToMove, warehosue)
		}
	}

}

func copySlice(src [][]byte) [][]byte {
	dest := make([][]byte, len(src))
	for i, inner := range src {
		if inner != nil {
			dest[i] = make([]byte, len(inner))
			copy(dest[i], inner)
		}
	}
	return dest
}

func canMove(blocksToMove map[Point]bool, delta Point, warehouse Grid) bool {
	for b := range blocksToMove {
		x, y := b.x, b.y
		nx, ny := x+delta.x, y+delta.y
		if isWall(nx, ny, warehouse) {
			return false
		}
	}
	return true
}

func simulateWide(input Input) Grid {
	dirMap := map[byte]Point{
		'<': {0, -1},
		'>': {0, 1},
		'^': {-1, 0},
		'v': {1, 0},
	}
	w, h := len(input.warehouse), len(input.warehouse[0])
	robotPos := getRobotsPos(input.warehouse)
	for _, dir := range input.directions {
		dx, dy := dirMap[dir].x, dirMap[dir].y
		nx, ny := robotPos.x+dx, robotPos.y+dy
		// fmt.Printf("dir: %v\n", string(dir))
		// printGrid(input.warehouse)
		// fmt.Println()
		if inBounds(nx, ny, w, h) && !isWall(nx, ny, input.warehouse) {
			if input.warehouse[nx][ny] == '.' {
				input.warehouse[nx][ny], input.warehouse[robotPos.x][robotPos.y] = input.warehouse[robotPos.x][robotPos.y], input.warehouse[nx][ny]
				robotPos.x = nx
				robotPos.y = ny
			} else if isBlock(nx, ny, input.warehouse) {
				blocksToMove := make(map[Point]bool)
				if isBlock(nx, ny, input.warehouse) {
					getBlocksToMove(Point{nx, ny}, dir, &blocksToMove, input.warehouse)
					if canMove(blocksToMove, dirMap[dir], input.warehouse) {
						copiedWarehouse := copySlice(input.warehouse)
						for b := range blocksToMove {
							copiedWarehouse[b.x][b.y] = '.'
						}
						for b := range blocksToMove {
							copiedWarehouse[b.x+dx][b.y+dy] = input.warehouse[b.x][b.y]
						}
						input.warehouse = copiedWarehouse
						for p := range blocksToMove {
							delete(blocksToMove, p)
						}
						input.warehouse[nx][ny] = input.warehouse[robotPos.x][robotPos.y]
						input.warehouse[robotPos.x][robotPos.y] = '.'
						robotPos.x = nx
						robotPos.y = ny

					}
				}
			}
		}
	}
	return input.warehouse
}

func sumGpsCoordinates(warehouse Grid) int {
	ans := 0
	for i, row := range warehouse {
		for j, cell := range row {
			if cell == 'O' || cell == '[' {
				ans += i*100 + j
			}
		}
	}
	return ans
}

func solve_part_one(data string) int {
	input := parseInput(data)
	warehouse := simulate(input)
	return sumGpsCoordinates(warehouse)
}

func solve_part_two(data string) int {
	input := parseInput(data)
	warehouse := constructWideWarehouse(input.warehouse)
	input.warehouse = warehouse
	simualtedWareHouse := simulateWide(input)
	return sumGpsCoordinates(simualtedWareHouse)
}

func main() {
	test := FileRead("../input/day15_1.test")
	prod := FileRead("../input/day15.prod")
	fmt.Println("Part_1 test: ", solve_part_one(test))
	fmt.Println("Part_1 prod: ", solve_part_one(prod))
	fmt.Println("Part_2 test: ", solve_part_two(test))
	fmt.Println("Part_2 prod: ", solve_part_two(prod))
}
