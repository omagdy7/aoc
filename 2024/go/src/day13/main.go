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

type Machine struct {
	Buttons [][]int64 // x, y cooefcients
	Prize   []int64   // x, y cooefcients
}

func parseInput(data string) []Machine {
	var machines []Machine
	lines := strings.Split(data, "\n\n")
	for _, line := range lines {
		var buttons [][]int64
		var prize []int64
		for _, subline := range strings.Split(line, "\n") {
			if len(subline) > 0 {
				line_splitted := strings.Split(subline, ": ")
				nums_with_ops := strings.Split(line_splitted[1], ", ") // X+99 Y+44
				if subline[0] == 'B' {
					var button []int64
					for _, coef := range nums_with_ops {
						coefInt, _ := strconv.Atoi(coef[2:])
						button = append(button, int64(coefInt))
					}
					buttons = append(buttons, button)
				} else {
					for _, coef := range nums_with_ops {
						coefInt, _ := strconv.Atoi(coef[2:])
						prize = append(prize, int64(coefInt))
					}
				}
			}
		}
		machines = append(machines, Machine{
			Buttons: buttons,
			Prize:   prize,
		})
	}
	return machines
}

// determinant calculates the determinant of a square matrix using recursion
func determinant(matrix [2][2]int64) int64 {
	return matrix[0][0]*matrix[1][1] - matrix[0][1]*matrix[1][0]
}

// replaceColumn replaces a column in a matrix with a given vector
func replaceColumn(matrix [2][2]int64, column [2]int64, colIndex int) [2][2]int64 {
	var newMatrix [2][2]int64
	for i := range matrix {
		newMatrix[i] = matrix[i]
		newMatrix[i][colIndex] = column[i]
	}
	return newMatrix
}

func cramerSolve(coefficients [2][2]int64, constants [2]int64) ([2]int64, error) {
	n := len(coefficients)

	detA := determinant(coefficients)
	if detA == 0 {
		return [2]int64{}, fmt.Errorf("no unique solution exists (determinant is zero)")
	}

	solutions := [2]int64{}
	for i := 0; i < n; i++ {
		modifiedMatrix := replaceColumn(coefficients, constants, i)
		detModified := determinant(modifiedMatrix)

		if detModified%detA != 0 {
			return [2]int64{}, fmt.Errorf("no integer solution exists")
		}
		solutions[i] = detModified / detA
	}

	return solutions, nil
}

func solve_part_one(data string) int64 {
	machines := parseInput(data)
	var coefficients [][]int64
	var constants []int64
	for _, machine := range machines {
		for i := 0; i < len(machine.Buttons); i++ {
			var machine_coeffs []int64
			for j := 0; j < len(machine.Buttons[0]); j++ {
				machine_coeffs = append(machine_coeffs, machine.Buttons[j][i])
			}
			coefficients = append(coefficients, machine_coeffs)
			constants = append(constants, machine.Prize[i])
		}
	}

	var ans int64 = 0

	for i := 0; i < len(constants); i += 2 {
		coeffs := [2][2]int64{{coefficients[i][0], coefficients[i][1]}, {coefficients[i+1][0], coefficients[i+1][1]}}
		consts := [2]int64{constants[i], constants[i+1]}
		sol, err := cramerSolve(coeffs, consts)
		if err == nil {
			ans += sol[0]*3 + sol[1]
		}
	}

	return ans
}

func solve_part_two(data string) int64 {
	machines := parseInput(data)
	var coefficients [][]int64
	var constants []int64
	for _, machine := range machines {
		for i := 0; i < len(machine.Buttons); i++ {
			var machine_coeffs []int64
			for j := 0; j < len(machine.Buttons[0]); j++ {
				machine_coeffs = append(machine_coeffs, (machine.Buttons[j][i]))
			}
			coefficients = append(coefficients, machine_coeffs)
			constants = append(constants, (machine.Prize[i] + 10000000000000))
		}
	}

	var ans int64 = 0

	for i := 0; i < len(constants); i += 2 {
		coeffs := [2][2]int64{{coefficients[i][0], coefficients[i][1]}, {coefficients[i+1][0], coefficients[i+1][1]}}
		consts := [2]int64{constants[i], constants[i+1]}
		sol, err := cramerSolve(coeffs, consts)
		if err == nil {
			ans += sol[0]*3 + sol[1]
		}
	}

	return ans
}

func main() {
	test := FileRead("../input/day13.test")
	prod := FileRead("../input/day13.prod")
	fmt.Println("Part_1 test: ", solve_part_one(test))
	fmt.Println("Part_1 prod: ", solve_part_one(prod))
	fmt.Println("Part_2 test: ", solve_part_two(test))
	fmt.Println("Part_2 prod: ", solve_part_two(prod))
}
