package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func Part2(input string) int64 {
	rangesStr := strings.Split(strings.TrimSpace(input), ",")
	var totalSum int64 = 0

	for _, r := range rangesStr {
		parts := strings.Split(r, "-")
		if len(parts) != 2 {
			continue
		}

		start, err1 := strconv.ParseInt(strings.TrimSpace(parts[0]), 10, 64)
		end, err2 := strconv.ParseInt(strings.TrimSpace(parts[1]), 10, 64)

		if err1 != nil || err2 != nil {
			fmt.Printf("Error parsing range: %s\n", r)
			continue
		}

		for i := start; i <= end; i++ {
			s := strconv.FormatInt(i, 10)
			n := len(s)

			isInvalid := false

			for l := 1; l <= n/2; l++ {
				if n%l == 0 {
					sub := s[:l]
					repeats := n / l

					if strings.Repeat(sub, repeats) == s {
						isInvalid = true
						break
					}
				}
			}
			if isInvalid {
				totalSum += i
			}
		}
	}

	return totalSum
}

func main() { 
	data, err := os.ReadFile("input-two.txt")
	if err != nil {
		panic(err)
	} else {
		
	}
	fmt.Println(Part2(string(data)))
}
