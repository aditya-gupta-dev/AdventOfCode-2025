package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func PartOne() {
	parseBatteries := func(line string) []int {
		var batteries []int = make([]int, 0, len(line))
		for i, text := range line {
			battery, err := strconv.Atoi(string(text))
			if err != nil {
				panic(err)
			}
			batteries[i] = battery
		}
		return batteries
	}

	solveBatteries := func(batteries []int) int {
		var maxJoltage int = 0
		var jolt int = 0

		for i := 0; i < len(batteries)-1; i++ {
			firstDigit := batteries[i]
			var secondDigit int = batteries[i+1]
			for j := i + 2; j < len(batteries); j++ {
				if batteries[j] > secondDigit {
					secondDigit = batteries[j]
				}
			}
			jolt = firstDigit*10 + secondDigit
			if maxJoltage < jolt {
				maxJoltage = jolt
			}
		}
		return maxJoltage
	}

	file, err := os.Open("input-three.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var total int = 0
	var scanner = bufio.NewScanner(file)

	for scanner.Scan() {
		var batteries = scanner.Text()
		total += solveBatteries(parseBatteries(batteries))
	}

	fmt.Println(total)
}

func PartTwo() {
	const required = 12

	parseBatteries := func(line string) []int {
		var batteries []int = make([]int, len(line))
		for i, text := range line {
			battery, err := strconv.Atoi(string(text))
			if err != nil {
				panic(err)
			}
			batteries[i] = battery
		}
		return batteries
	}

	solveBatteries := func(batteries []int) string {
		var result []byte = make([]byte, 0, required)
		var start int = 0

		for i := 0; i < required; i++ {
			remaining := required - i
			end := len(result) - remaining + 1
			maxDigit := batteries[start]
			pos := start

			for j := start; j < end; j++ {
				if batteries[j] > maxDigit {
					maxDigit = batteries[j]
					pos = j
				}
			}

			result = append(result, byte(maxDigit))
			start = pos + 1
		}
		return string(result)
	}

	file, err := os.Open("input-three.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var total int64 = 0
	var scanner = bufio.NewScanner(file)

	for scanner.Scan() {
		var batteries = scanner.Text()
		var joltage int64 = 0
		joltageStr := solveBatteries(parseBatteries(batteries))
		for _, ch := range joltageStr {
			joltage = joltage*10 + int64(ch-'0')
		}
		total += joltage
	}

	fmt.Println(total)
}

func main() {
	PartOne()
}
