package main

import (
	"io/ioutil"
	"strconv"
	"strings"
)

// returns list of rocks, min and max koordinates
func getRocks() ([][2]int, [2]int, [2]int) {
	rocks := make([][2]int, 0)
	minCoordinate := [2]int{5000, 5000}
	maxCoordinate := [2]int{0, 0}

	content, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(content), "\n")
	for _, line := range lines {
		coordinates := strings.Split(line, " -> ")
		for i := 0; i < len(coordinates)-1; i++ {
			x1, y1 := coordinateFromString(coordinates[i])
			x2, y2 := coordinateFromString(coordinates[i+1]) // We could save this for the next run but who cares :D

			for x := min(x1, x2); x <= max(x1, x2); x++ {
				for y := min(y1, y2); y <= max(y1, y2); y++ {
					rocks = append(rocks, [2]int{x, y})

					if x > maxCoordinate[0] {
						maxCoordinate[0] = x
					}
					if y > maxCoordinate[1] {
						maxCoordinate[1] = y
					}
					if x < minCoordinate[0] {
						minCoordinate[0] = x
					}
					if y < minCoordinate[1] {
						minCoordinate[1] = y
					}
				}
			}
		}
	}

	if maxCoordinate[0] < 500 {
		maxCoordinate[0] = 500
	}
	if minCoordinate[1] > 0 {
		minCoordinate[1] = 0
	}

	return rocks, minCoordinate, maxCoordinate
}

func coordinateFromString(coordinate string) (int, int) {
	splitted := strings.Split(coordinate, ",")

	x, err := strconv.Atoi(splitted[0])
	if err != nil {
		panic(err)
	}
	y, err := strconv.Atoi(splitted[1])
	if err != nil {
		panic(err)
	}

	return x, y
}

func max(a int, b int) int {
	if a > b {
		return a
	}

	return b
}

func min(a int, b int) int {
	if a < b {
		return a
	}

	return b
}

func toMap(rocks [][2]int, min [2]int, max [2]int) [][]rune {
	dx := max[0] - min[0] + 1
	dy := max[1] - min[1] + 1

	m := make([][]rune, dx)

	// prefill map
	for x := range m {
		m[x] = make([]rune, dy)
		for y := range m[x] {
			m[x][y] = '.'
		}
	}

	for _, rock := range rocks {
		m[rock[0]-min[0]][rock[1]-min[1]] = '#'
	}

	return m
}

func addFloor(m [][]rune) ([][]rune, int) {
	addRows := 500
	newM := make([][]rune, len(m)+(addRows*2))

	for i := 0; i < addRows; i++ {
		newM[i] = make([]rune, len(m[0])+2)
		for j := 0; j < len(m[0])+1; j++ {
			newM[i][j] = '.'
		}
		newM[i][len(m[0])+1] = '#'
	}

	for x := range m {
		newM[x+addRows] = m[x]
		newM[x+addRows] = append(newM[x+addRows], '.')
		newM[x+addRows] = append(newM[x+addRows], '#')
	}

	for i := len(m) + addRows; i < len(newM); i++ {
		newM[i] = make([]rune, len(m[0])+2)
		for j := 0; j < len(m[0])+1; j++ {
			newM[i][j] = '.'
		}
		newM[i][len(m[0])+1] = '#'
	}

	return newM, addRows
}
