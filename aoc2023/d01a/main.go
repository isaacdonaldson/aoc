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
