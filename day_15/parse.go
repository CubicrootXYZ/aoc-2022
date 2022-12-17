package main

import (
	"os"
	"strconv"
	"strings"
)

type Sensor struct {
	Pos    [2]int
	Beacon [2]int
}

func (s *Sensor) Dist() int {
	return abs(s.Pos[0]-s.Beacon[0]) + abs(s.Pos[1]-s.Beacon[1])
}

func getSensors() ([]Sensor, [2]int, [2]int) {
	sensors := make([]Sensor, 0)
	maxC := [2]int{0, 0}
	minC := [2]int{5000, 5000}

	content, err := os.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}

	for _, line := range strings.Split(string(content), "\n") {
		splitted := strings.Split(line, " ")

		s_x, err := strconv.Atoi(strings.TrimSuffix(strings.TrimPrefix(splitted[2], "x="), ","))
		if err != nil {
			panic(err)
		}
		s_y, err := strconv.Atoi(strings.TrimSuffix(strings.TrimPrefix(splitted[3], "y="), ":"))
		if err != nil {
			panic(err)
		}

		b_x, err := strconv.Atoi(strings.TrimSuffix(strings.TrimPrefix(splitted[8], "x="), ","))
		if err != nil {
			panic(err)
		}
		b_y, err := strconv.Atoi(strings.TrimSuffix(strings.TrimPrefix(splitted[9], "y="), ":"))
		if err != nil {
			panic(err)
		}

		s := Sensor{
			Pos:    [2]int{s_x, s_y},
			Beacon: [2]int{b_x, b_y},
		}

		if s.Pos[1] > 2000000+s.Dist()+2 || s.Pos[1] < 2000000-s.Dist()-2 {
			// Ignore beacons far away from our line
			continue
		}

		sensors = append(sensors, s)
	}

	return sensors, minC, maxC
}

func cntPart1(sensors []Sensor) int {
	cnt := 0
	seen := make(map[[2]int]bool)

	for _, s := range sensors {
		for _, cs := range coordinatesInRange(s.Pos, s.Dist()) {
			if cs[1] == 2000000 {
				if !seen[cs] && !(cs[0] == s.Pos[0] && cs[1] == s.Pos[1]) && !(cs[0] == s.Beacon[0] && cs[1] == s.Beacon[1]) {
					//fmt.Println(cs)
					//fmt.Println(seen)
					cnt += 1
				}
				seen[cs] = true
			}
		}
	}

	return cnt
}

func abs(i int) int {
	if i < 0 {
		return i * -1
	}

	return i
}

func coordinatesInRange(point [2]int, r int) [][2]int {
	cs := make([][2]int, 1)
	cs[0] = point

	// I am sure there is a more clever approach to this
	for y := point[1] - r; y <= point[1]+r; y++ {
		if y != 2000000 {
			// ignore points not in our line
			continue
		}

		for x := point[0] - r; x <= point[0]+r; x++ {
			if abs(point[0]-x)+abs(point[1]-y) <= r {
				cs = append(cs, [2]int{x, y})
			}
		}
	}

	return cs
}
