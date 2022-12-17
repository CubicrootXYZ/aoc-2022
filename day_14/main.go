package main

import "fmt"

func main() {
	//part1()
	part2()
}

func part1() {
	rocks, min, max := getRocks()
	fmt.Printf("Found %d rocks within [%d, %d] [%d, %d]", len(rocks), min[0], min[1], max[0], max[1])

	m := toMap(rocks, min, max)
	printM(m)

	pp := getPouringPoint(min)

	i := 0
	for {
		i++
		finished := poorSingleSand(m, pp)
		// debug printM(m)
		// fmt.Println()

		if finished {
			break
		}
	}

	dumpM(m, "out1.txt")
	printM(m)
	fmt.Printf("Found %d sand\n\n", countSand(m)-1) // subtract the one that actually would fall of the map
}

func part2() {
	// to high 25257
	// to low  25247
	// 25252
	// 25250
	rocks, min, max := getRocks()
	fmt.Printf("Found %d rocks within [%d, %d] [%d, %d]", len(rocks), min[0], min[1], max[0], max[1])

	m := toMap(rocks, min, max)
	m, addPpX := addFloor(m)
	printM(m)

	pp := getPouringPoint(min)
	pp[0] += addPpX

	for {
		if poorSingleSand(m, pp) {
			panic("map to small")
		}
		//printM(m)
		//fmt.Println()

		if sourceBlocked(m, pp) {
			break
		}
	}

	dumpM(m, "out2.txt")
	fmt.Printf("Found %d sand", countSand(m))
}

func getPouringPoint(min [2]int) [2]int {
	return [2]int{500 - min[0], min[1]}
}

func poorSingleSand(m [][]rune, pp [2]int) bool {
	s := pp
	next_pos := s

	for {
		s = next_pos

		rest := true
		for _, delta := range [][2]int{{0, 0}, {0, 1}, {-1, 0}, {2, 0}} {
			next_pos[1] += delta[1]
			next_pos[0] += delta[0]
			// debug fmt.Print(next_pos)
			// Todo check if pos is in map
			applied, finished := applySand(m, next_pos, s)
			if finished {
				return true
			}
			if applied {
				rest = false
				break
			}
		}

		if rest {
			break // we came to rest
		}
	}

	return false
}

func applySand(m [][]rune, pos [2]int, pos_old [2]int) (bool, bool) {
	if len(m) <= pos[0] || len(m[0]) <= pos[1] || pos[0] < 0 || pos[1] < 0 {
		return false, true
	}

	if m[pos[0]][pos[1]] == '.' {
		m[pos_old[0]][pos_old[1]] = '.'
		m[pos[0]][pos[1]] = 'o'
		return true, false
	}
	return false, false
}

func countSand(m [][]rune) int {
	cnt := 0

	for x := range m {
		for y := range m[x] {
			if m[x][y] == 'o' {
				cnt += 1
			}
		}
	}

	return cnt
}

func sourceBlocked(m [][]rune, pp [2]int) bool {
	return m[pp[0]][pp[1]] == 'o'
}
