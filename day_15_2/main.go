package main

import "fmt"

func main() {
	// the ranges are to big to actually make a 2d map, we will run out of memory
	// for p2 it is not possible to just iterate over any field, maybe iterate through all lines and use ranges? find a
	// gap in them? Go has no range type so make our own?
	sensors, min, max := getSensors()
	fmt.Printf("Found %d sensors\n", len(sensors))
	fmt.Print(min)
	fmt.Print(max)

	m := cntPart1(sensors)
	//printM(m)
	fmt.Printf("There are %d cannot be positions", m)
}
