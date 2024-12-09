package main

import (
	"fmt"
	"os"
	"strconv"
)

func InsertAt(arr *[]FileBlock, index int, value FileBlock) error {
	if index < 0 || index > len(*arr) {
		return fmt.Errorf("index out of range")
	}
	*arr = append((*arr)[:index], append([]FileBlock{value}, (*arr)[index:]...)...)
	return nil
}

func RemoveAt(arr *[]FileBlock, index int) (FileBlock, error) {
	if index < 0 || index >= len(*arr) {
		return FileBlock{}, fmt.Errorf("index out of range")
	}
	ele := (*arr)[index]
	*arr = append((*arr)[:index], (*arr)[index+1:]...)
	return ele, nil
}

func FileRead(path string) string {
	file, err := os.ReadFile(path)
	if err != nil {
		fmt.Println("Couldn't Read file: ", err)
	}
	return string(file)

}

func parseInput(data string) string {
	return data
}

func genereateBlockMap(diskMap string) []string {
	var blockMap []string
	id := 0
	for i, ch := range diskMap {
		rep, _ := strconv.Atoi(string(ch))
		if i%2 == 0 {
			for i := 0; i < rep; i++ {
				blockMap = append(blockMap, strconv.Itoa(id))
			}
			id += 1
		} else {
			for i := 0; i < rep; i++ {
				blockMap = append(blockMap, ".")
			}
		}
	}
	return blockMap
}

func genereateFileBlocks(diskMap string) []FileBlock {
	var fileBlocks []FileBlock
	id := 0
	for i, ch := range diskMap {
		rep, _ := strconv.Atoi(string(ch))
		if i%2 == 0 {
			block := FileBlock{id: id, space: rep}
			fileBlocks = append(fileBlocks, block)
			id += 1
		} else {
			block := FileBlock{id: -1, space: rep}
			if rep > 0 {
				fileBlocks = append(fileBlocks, block)
			}
		}
	}
	return fileBlocks
}

func moveBlocks(blockMap []string) {
	i := 0
	j := len(blockMap) - 1
	for i < j {
		if blockMap[i] == "." {
			if blockMap[j] != "." {
				blockMap[i], blockMap[j] = blockMap[j], blockMap[i]
				i++
			}
			j--
		} else {
			i++
		}
	}
}

type FileBlock struct {
	id    int // id -1 will be the free block '.'
	space int
}

//        i = 1                                                                                                          j=12
// [{0 2} {-1 3} {-1,1} {1 3} {-1 3} {2 1} {-1 3} {3 3} {-1 1} {4 2} {-1 1} {5 4} {-1 1} {6 4} {-1 1} {7 3} {-1 1} {8 4} {9 2}]

func moveBlocksPartTwo(fileBlocks []FileBlock) []string {
	for i := 0; i < len(fileBlocks); i++ {
		if fileBlocks[i].id != -1 {
			continue
		}
		for j := len(fileBlocks) - 1; j >= i; j-- {
			if fileBlocks[j].id == -1 {
				continue
			}
			if fileBlocks[i].space == fileBlocks[j].space {
				fileBlocks[i], fileBlocks[j] = fileBlocks[j], fileBlocks[i]
				i++
				j--
				break
			} else if fileBlocks[i].space > fileBlocks[j].space {
				_ = InsertAt(&fileBlocks, i+1, FileBlock{id: -1, space: fileBlocks[i].space - fileBlocks[j].space})
				spaceJ := fileBlocks[j+1].space
				fileBlocks[i], fileBlocks[j+1] = fileBlocks[j+1], fileBlocks[i]
				fileBlocks[j+1].id = -1
				fileBlocks[j+1].space = spaceJ
				break
			}
		}
	}
	var blocks []string
	for _, block := range fileBlocks {
		if block.id == -1 {
			for i := 0; i < block.space; i++ {
				blocks = append(blocks, ".")
			}
		} else {
			for i := 0; i < block.space; i++ {
				blocks = append(blocks, strconv.Itoa(block.id))
			}
		}
	}
	return blocks
}

func checkSum(blockMap []string) int64 {
	var sum int64 = 0
	for i, ch := range blockMap {
		if ch == "." {
			continue
		}
		id, _ := strconv.Atoi(ch)
		sum += int64(id) * int64(i)
	}
	return sum
}
func solve_part_one(data string) int64 {
	diskMap := parseInput(data)
	blockMap := genereateBlockMap(diskMap)
	moveBlocks(blockMap)
	return checkSum(blockMap)
}

func solve_part_two(data string) int64 {
	diskMap := parseInput(data)
	fileBlocks := genereateFileBlocks(diskMap)
	blocks := moveBlocksPartTwo(fileBlocks)
	return checkSum(blocks)
}

func main() {
	test := FileRead("../input/day9.test")
	prod := FileRead("../input/day9.prod")
	fmt.Println("Part_1 test: ", solve_part_one(test))
	fmt.Println("Part_1 prod: ", solve_part_one(prod))
	fmt.Println("Part_2 test: ", solve_part_two(test))
	fmt.Println("Part_2 prod: ", solve_part_two(prod))
}
