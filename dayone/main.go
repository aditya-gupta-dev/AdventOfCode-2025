package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func PartOne() {
	data, err := os.ReadFile("input-one.txt")
	if err != nil {
		panic(err)
	}
	var rotation int = 50
	var lines []string = strings.Split(string(data), "\n")
	var zeroCounter int = 0

	for _, line := range lines {
		if len(line) == 0 {
			continue
		}

		var direction = rune(line[0])
		degree, err := strconv.Atoi(line[1:])
		if err != nil {
			panic(err)
		}

		switch direction {
		case 'L':
			rotation = (rotation - degree) % 100
			if rotation < 0 {
				rotation += 100
			}
		case 'R':
			rotation = (rotation + degree) % 100
		}

		if rotation == 0 {
			zeroCounter += 1
		}
	}
	fmt.Println(zeroCounter)
}

func main() {
	PartOne()
}
