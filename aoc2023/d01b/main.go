package main

import (
	"fmt"
	"os"
	"strings"
	"unicode"
)

func check(err error) {
	if err != nil {
		panic(err)
	}
}

func readFile(path string) string {
	contents, err := os.ReadFile(path)
	check(err)

	return string(contents)
}

func main() {
	inputs := readFile("input.txt")

	replacer := strings.NewReplacer(
		// Compound numbers
		"oneight", "18",
		"threeight", "38",
		"fiveight", "58",
		"sevenine", "79",
		"nineight", "98",
		"twone", "21",
		"eightwo", "82",
		"eighthree", "83",
		// Single numbers
		"one", "1",
		"two", "2",
		"three", "3",
		"four", "4",
		"five", "5",
		"six", "6",
		"seven", "7",
		"eight", "8",
		"nine", "9")

	inputs = replacer.Replace(inputs)

	fmt.Println(inputs)

	calculation_total := 0

	for _, line := range strings.Split(inputs, "\n") {
		is_first := true
		first_num := 0
		last_num := 0
		for _, r := range line {
			if unicode.IsDigit(r) {
				if is_first {
					first_num = int(r - '0')
					is_first = false
				}

				last_num = int(r - '0')
			}
		}

		calculation_total += first_num*10 + last_num
	}

	fmt.Println(calculation_total)
}
