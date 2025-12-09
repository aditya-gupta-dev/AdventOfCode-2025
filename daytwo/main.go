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

// package main

// import (
// 	"fmt"
// 	"os"
// 	"strconv"
// 	"strings"
// )

// type Rng struct {
// 	start int64
// 	end   int64
// }

// func main() {
// 	data, err := os.ReadFile("input-two.txt")
// 	if err != nil {
// 		panic(err)
// 	}
// 	line := strings.TrimSpace(string(data))
// 	if line == "" {
// 		fmt.Println(0)
// 		return
// 	}

// 	parts := strings.Split(line, ",")
// 	var ranges []Rng
// 	var maxEnd int64 = 0
// 	for _, p := range parts {
// 		p = strings.TrimSpace(p)
// 		if p == "" {
// 			continue
// 		}
// 		se := strings.Split(p, "-")
// 		if len(se) != 2 {
// 			panic("bad range: " + p)
// 		}
// 		s, _ := strconv.ParseInt(se[0], 10, 64)
// 		e, _ := strconv.ParseInt(se[1], 10, 64)
// 		if e < s {
// 			s, e = e, s
// 		}
// 		ranges = append(ranges, Rng{s, e})
// 		if e > maxEnd {
// 			maxEnd = e
// 		}
// 	}

// 	var total int64 = 0

// 	// Precompute pow10 up to 19 (safe for int64)
// 	pow10 := func(n int) int64 {
// 		r := int64(1)
// 		for i := 0; i < n; i++ {
// 			r *= 10
// 		}
// 		return r
// 	}

// 	// For each block length (1..18). We pick a conservative upper bound:
// 	// blockLen up to 18 is more than enough because even 1 repeated 19 times would overflow int64.
// 	for blockLen := 1; blockLen <= 18; blockLen++ {
// 		minX := pow10(blockLen - 1) // no leading zero (except blockLen==1 -> 1..9)
// 		maxX := pow10(blockLen) - 1
// 		mul := pow10(blockLen) // multiplier to append another block

// 		for x := minX; x <= maxX; x++ {
// 			var repeated int64 = 0
// 			// build repeated numbers progressively: X, XX, XXX, ...
// 			// but we only start checking from repeatCount >= 2
// 			for repeatCount := 1; ; repeatCount++ {
// 				// before doing repeated = repeated * mul + x, check overflow and limit
// 				if repeated > 0 {
// 					// check overflow: repeated*mul + x <= maxEnd
// 					if repeated > maxEnd/mul {
// 						// any further repeats will be even larger -> stop this x
// 						break
// 					}
// 				}
// 				repeated = repeated*mul + int64(x)

// 				if repeated > maxEnd {
// 					// too large for any range — stop repeating this x
// 					break
// 				}

// 				if repeatCount >= 2 {
// 					// check whether repeated falls into any range; add only once
// 					if inAnyRange(repeated, ranges) {
// 						total += repeated
// 						// do not break; a larger repeated (more repeats) may also be in ranges
// 						// but we should NOT add the same numeric value twice (we won't)
// 					}
// 				}
// 				// continue building more repeats until we exceed maxEnd or overflow
// 			}
// 		}
// 	}

// 	fmt.Println(total)
// }

// func inAnyRange(val int64, ranges []Rng) bool {
// 	for _, r := range ranges {
// 		if val >= r.start && val <= r.end {
// 			return true
// 		}
// 	}
// 	return false
// }
