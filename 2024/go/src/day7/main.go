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

type Op = string

const (
	Plus Op = "+"
	Mul  Op = "*"
)

var Ops = [2]Op{Plus, Mul}

type Calibration struct {
	target int
	vals   []int
}

type Expression = []string

// 81 + 57 * 52

func (c Calibration) String() string {
	valsStr := strings.Trim(strings.Join(strings.Fields(fmt.Sprint(c.vals)), ", "), "[]")
	return fmt.Sprintf("Calibration(target: %d, vals: [%s])", c.target, valsStr)
}

func parseInput(data string) []Calibration {
	lines := strings.Split(data, "\n")

	var cals []Calibration

	for _, line := range lines {
		if len(line) > 0 {
			cal := strings.Split(line, ": ")
			target, _ := strconv.Atoi(cal[0])
			var vals []int
			for _, num := range strings.Split(cal[1], " ") {
				val, _ := strconv.Atoi(num)
				vals = append(vals, val)
			}
			calibration := Calibration{
				target: target,
				vals:   vals,
			}
			cals = append(cals, calibration)
		}
	}

	return cals
}

func generatePossibleExpressions(opsLeft int, expr *Expression, expressions *[]Expression) {
	if opsLeft == 0 {
		comb := make([]string, len(*expr))
		copy(comb, *expr)
		*expressions = append(*expressions, comb)
	} else {
		for _, op := range Ops {
			// choose
			*expr = append(*expr, op)

			// explore
			generatePossibleExpressions(opsLeft-1, expr, expressions)

			// unchoose
			*expr = (*expr)[:len(*expr)-1]
		}
	}

}

func eval(vals []int, expressions *[]Expression) int {
	for i := len(*expressions) - 1; i >= 0; i-- {
		for j, op := range (*expressions)[i] {
			left := vals[j-1]
			right := vals[j]
			evaluation := 0
			if op == Plus {
				evaluation := left + right
			} else if op == Mul {
				evaluation := left * right
			}
		}
	}

	return 5
}

func solve_part_one(data string) int {
	calibrations := parseInput(data)
	ans := 0
	for _, calibration := range calibrations {
		var expressions []Expression
		generatePossibleExpressions((len(calibration.vals) - 1), &[]string{}, &expressions)
		if eval(calibration.vals, &expressions) == calibration.target {
			ans += calibration.target
		}
		fmt.Printf("expressions: %v\n", expressions)
		expressions = []Expression{}

	}
	return ans
}

func solve_part_two(data string) int {
	return 5
}

func main() {
	test := FileRead("../input/day7.test")
	// prod := FileRead("../input/day7.prod")
	fmt.Println("Part_1 test: ", solve_part_one(test))
	// fmt.Println("Part_1 prod: ", solve_part_one(prod))
	// fmt.Println("Part_2 test: ", solve_part_two(test))
	// fmt.Println("Part_2 prod: ", solve_part_two(prod))
}
