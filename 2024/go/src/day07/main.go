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
	Plus   Op = "+"
	Mul    Op = "*"
	Concat Op = "||"
)

var OpsOne = []Op{Plus, Mul}
var OpsTwo = []Op{Plus, Mul, Concat}

type Calibration struct {
	target int
	vals   []int
}

type Expression = []string

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

func generatePossibleExpressions(opsLeft int, expr *Expression, expressions *[]Expression, Ops []string) {
	if opsLeft == 0 {
		comb := make([]string, len(*expr))
		copy(comb, *expr)
		*expressions = append(*expressions, comb)
	} else {
		for _, op := range Ops {
			// choose
			*expr = append(*expr, op)

			// explore
			generatePossibleExpressions(opsLeft-1, expr, expressions, Ops)

			// unchoose
			*expr = (*expr)[:len(*expr)-1]
		}
	}

}

func removeElement(s []int, i int) ([]int, error) {
	if i >= len(s) || i < 0 {
		return nil, fmt.Errorf("Index is out of range. Index is %d with slice length %d", i, len(s))
	}
	return append(s[:i], s[i+1:]...), nil
}

func removeElementString(s []string, i int) ([]string, error) {
	if i >= len(s) || i < 0 {
		return nil, fmt.Errorf("Index is out of range. Index is %d with slice length %d", i, len(s))
	}
	return append(s[:i], s[i+1:]...), nil
}

func evalPartOne(vals []int, expression *Expression) int {
	for _, op := range *expression {
		left := vals[0]
		right := vals[1]
		if op == Mul {
			vals, _ = removeElement(vals, 0)
			vals[0] = left * right
		} else if op == Plus {
			vals, _ = removeElement(vals, 0)
			vals[0] = left + right
		}
	}

	return vals[0]
}
func evalPartTwo(vals []int, expression *Expression) int {
	for _, op := range *expression {
		left := vals[0]
		right := vals[1]
		if op == Mul {
			vals, _ = removeElement(vals, 0)
			vals[0] = left * right
		} else if op == Plus {
			vals, _ = removeElement(vals, 0)
			vals[0] = left + right
		} else if op == "||" {
			vals, _ = removeElement(vals, 0)
			// vals[0], _ = strconv.Atoi(strconv.Itoa(left) + strconv.Itoa(right))
			multiplier := 1
			for temp := right; temp > 0; temp /= 10 {
				multiplier *= 10
			}
			vals[0] = left*multiplier + right
		}
	}

	// fmt.Printf("expression: %v\n", (*expression))
	// fmt.Printf("vals: %v\n", vals)

	return vals[0]
}
func solve_part_one(data string) int64 {
	calibrations := parseInput(data)
	var ans int64 = 0
	for _, calibration := range calibrations {
		var expressions []Expression
		generatePossibleExpressions((len(calibration.vals) - 1), &[]string{}, &expressions, OpsOne)
		for _, expr := range expressions {
			copy_vals := make([]int, len(calibration.vals))
			copy(copy_vals, calibration.vals)
			if evalPartOne(copy_vals, &expr) == calibration.target {
				// fmt.Printf("expr: %v\n", expr)
				ans += int64(calibration.target)
				break
			}
		}
		// fmt.Printf("expressions: %v\n", expressions)
		expressions = []Expression{}
	}
	return ans
}

func solve_part_two(data string) int64 {
	calibrations := parseInput(data)
	var ans int64 = 0
	for _, calibration := range calibrations {
		var expressions []Expression
		generatePossibleExpressions((len(calibration.vals) - 1), &[]string{}, &expressions, OpsTwo)
		for _, expr := range expressions {
			copy_vals := make([]int, len(calibration.vals))
			copy(copy_vals, calibration.vals)
			if evalPartTwo(copy_vals, &expr) == calibration.target {
				// fmt.Printf("expr: %v\n", expr)
				ans += int64(calibration.target)
				break
			}
		}
		expressions = []Expression{}
	}
	return ans
}

func main() {
	test := FileRead("../input/day07.test")
	prod := FileRead("../input/day07.prod")
	fmt.Println("Part_1 test: ", solve_part_one(test))
	fmt.Println("Part_1 prod: ", solve_part_one(prod))
	fmt.Println("Part_2 test: ", solve_part_two(test))
	fmt.Println("Part_2 prod: ", solve_part_two(prod))
}
