package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Range struct {
	start int64
	end   int64
}

func main() {
	data, err := os.ReadFile("input-two.txt")
	if err != nil {
		panic(err)
	}

	line := strings.TrimSpace(string(data))
	parts := strings.Split(line, ",")

	var ranges []Range
	for _, p := range parts {
		se := strings.Split(p, "-")
		start, _ := strconv.ParseInt(se[0], 10, 64)
		end, _ := strconv.ParseInt(se[1], 10, 64)
		ranges = append(ranges, Range{start, end})
	}

	var sum int64 = 0

	for length := 1; length <= 9; length++ {
		start := pow10(length - 1)
		end := pow10(length) - 1
		for x := start; x <= end; x++ {
			val := repeatTwice(x, length)
			for _, r := range ranges {
				if val >= r.start && val <= r.end {
					sum += val
				}
			}
		}
	}

	fmt.Println(sum)
}

func repeatTwice(x int64, length int) int64 {
	return x*pow10(length) + x
}

func pow10(n int) int64 {
	result := int64(1)
	for i := 0; i < n; i++ {
		result *= 10
	}
	return result
}
