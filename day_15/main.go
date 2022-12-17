package main

import "fmt"

func main() {
	// the ranges are to big to actually make a 2d map, we will run out of memory
	sensors, min, max := getSensors()
	fmt.Printf("Found %d sensors\n", len(sensors))
	fmt.Print(min)
	fmt.Print(max)

	m := cntPart1(sensors)
	//printM(m)
	fmt.Printf("There are %d cannot be positions", m)
}
