package main

import (
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/beefsack/go-astar"
)

type tile struct {
	Elevation  int
	Neighboors []*tile
}

func (t *tile) PathNeighbors() []astar.Pather {
	p := make([]astar.Pather, 0)
	for _, tile := range t.Neighboors {
		p = append(p, tile)
	}
	return p
}

func (t *tile) PathNeighborCost(to astar.Pather) float64 {
	return 1
}

func (t *tile) PathEstimatedCost(to astar.Pather) float64 {
	return 1
}

func main() {
	theMap, start, end := parseInput()
	link_map(theMap) // Could also be done in flight with the PathNeighbors() func

	// Part 1
	// Feels like cheating using a package for this :/
	_, distance, found := astar.Path(start, end)
	if !found {
		panic("Could not find path")
	}
	fmt.Printf("Min dist %f\n", distance)

	// Part 2
	dists := make([]float64, 1)
	dists[0] = distance
	for x := range theMap {
		for y := range theMap[x] {
			if theMap[x][y].Elevation > 0 {
				continue
			}
			_, distance, found := astar.Path(theMap[x][y], end)
			if !found {
				fmt.Println("Could not find path")
				continue
			}
			dists = append(dists, distance)
		}
	}
	fmt.Printf("Min dist II: %f\n", min(dists))
}

func link_map(theMap [][]*tile) {
	for x := range theMap {
		for y := range theMap[x] {
			current_tile := theMap[x][y]
			neighbors := [][]int{}
			if x > 0 {
				neighbors = append(neighbors, []int{x - 1, y})
			}
			if y > 0 {
				neighbors = append(neighbors, []int{x, y - 1})
			}
			if x < len(theMap)-1 {
				neighbors = append(neighbors, []int{x + 1, y})
			}
			if y < len(theMap[x])-1 {
				neighbors = append(neighbors, []int{x, y + 1})
			}

			for _, n := range neighbors {
				neighbor := theMap[n[0]][n[1]]
				if neighbor.Elevation <= current_tile.Elevation+1 {
					current_tile.Neighboors = append(current_tile.Neighboors, neighbor)
				}
			}
		}
	}
}

func parseInput() ([][]*tile, *tile, *tile) {
	content, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	rows := strings.Split(string(content), "\n")

	theMap := make([][]*tile, len(rows))

	var start *tile
	var end *tile

	for r := range rows {
		theMap[r] = make([]*tile, len(rows[r]))
		for c, j := range rows[r] {
			tile := tile{
				Elevation: int(j - 'a'),
			}

			if j == 'S' {
				tile.Elevation = 0
				start = &tile
			}
			if j == 'E' {
				tile.Elevation = 25
				end = &tile
			}

			theMap[r][c] = &tile
		}
	}

	return theMap, start, end
}

func min(list []float64) float64 {
	min := 100000.0
	for _, v := range list {
		if v < min {
			min = v
		}
	}

	return min
}
