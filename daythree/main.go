package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func PartTwo() int64 { 
	data, err := os.ReadFile("input-three.txt")
	if err != nil { 
		panic(err)
	}
	input := string(data)
	scanner := bufio.NewScanner(strings.NewReader(strings.TrimSpace(input)))
	var totalJoltage int64 = 0
	
	const k = 12 

	for scanner.Scan() {
		bank := scanner.Text()
		if len(bank) == 0 {
			continue
		}

		toDrop := len(bank) - k
		if toDrop < 0 {
			toDrop = 0
		}

		var stack []byte
		
		for i := 0; i < len(bank); i++ {
			digit := bank[i]
			
			for toDrop > 0 && len(stack) > 0 && digit > stack[len(stack)-1] {
				stack = stack[:len(stack)-1]
				toDrop--
			}
			stack = append(stack, digit)
		}

		if toDrop > 0 {
			stack = stack[:len(stack)-toDrop]
		}
		
		if len(stack) > k {
			stack = stack[:k]
		}

		val, err := strconv.ParseInt(string(stack), 10, 64)
		if err != nil {
			fmt.Printf("Failed to parse %s: %v\n", string(stack), err)
			continue
		}
		
		totalJoltage += val
	}

	return totalJoltage
}

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


func main() { 
	fmt.Println(PartTwo())
}
