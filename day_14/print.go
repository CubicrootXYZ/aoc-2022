package main

import (
	"fmt"
	"io/ioutil"
)

func printM(m [][]rune) {
	for y := range m[0] {
		for x := range m {
			fmt.Printf("%s", string(m[x][y]))
		}
		fmt.Println()
	}
	fmt.Println()
}

func dumpM(m [][]rune, name string) {
	out := ""
	for y := range m[0] {
		for x := range m {
			out += fmt.Sprintf("%s", string(m[x][y]))
		}
		out += "\n"
	}

	ioutil.WriteFile(name, []byte(out), 0644)
}
