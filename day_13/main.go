package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"reflect"
	"sort"
	"strings"
)

func main() {
	p1()
	p2()
}

func p1() {
	content, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(content), "\n")
	sum := 0
	for i := 0; i <= (len(lines) / 3); i++ {
		l1 := []interface{}{}
		err := json.Unmarshal([]byte(lines[i*3]), &l1)
		if err != nil {
			panic(err)
		}
		l2 := []interface{}{}
		err = json.Unmarshal([]byte(lines[(i*3)+1]), &l2)
		if err != nil {
			panic(err)
		}

		if isOrdered2(l1, l2) == 1 {
			fmt.Printf("%d is ordered", i+1)
			fmt.Println(i + 1)
			sum += i + 1
		}
		fmt.Println()
	}

	fmt.Printf("Summed indizes: %d", sum)
}

func p2() {
	content, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}

	packets := make([]interface{}, 2)
	packets[0] = []interface{}{[]interface{}{2.0}}
	packets[1] = []interface{}{[]interface{}{6.0}}

	lines := strings.Split(string(content), "\n")
	for i := 0; i <= (len(lines) / 3); i++ {
		l1 := []interface{}{}
		err := json.Unmarshal([]byte(lines[i*3]), &l1)
		if err != nil {
			panic(err)
		}
		l2 := []interface{}{}
		err = json.Unmarshal([]byte(lines[(i*3)+1]), &l2)
		if err != nil {
			panic(err)
		}

		packets = append(packets, l1)
		packets = append(packets, l2)
	}

	sort.SliceStable(packets, func(i, j int) bool {
		return isOrdered2(packets[i], packets[j]) == 1
	})

	prod := make([]int, 0)
	i := 1
	for _, p := range packets {
		if p1, ok := p.([]interface{}); ok && len(p1) == 1 {
			if p2, ok := p1[0].([]interface{}); ok && len(p2) == 1 {
				if p2[0] == 2.0 || p2[0] == 6.0 {
					prod = append(prod, i)
				}
			}
		}

		i += 1
	}
	fmt.Printf("Val is %d", prod[0]*prod[1])
}

func isOrdered2(l1 interface{}, l2 interface{}) int {
	fmt.Printf("Compare %v with %v\n", l1, l2)

	n1, n1ok := l1.(float64)
	n2, n2ok := l2.(float64)
	if n1ok && n2ok {
		if n1 < n2 {
			return 1
		} else if n1 > n2 {
			return 0
		}
		return 2
	}
	if n1ok && !n2ok {
		return isOrdered2([]interface{}{l1}, l2)
	}
	if !n1ok && n2ok {
		return isOrdered2(l1, []interface{}{l2})
	}

	sn1 := l1.([]interface{})
	sn2 := l2.([]interface{})
	for i := 0; i < min(len(sn1), len(sn2)); i++ {
		res := isOrdered2(sn1[i], sn2[i])
		if res != 2 {
			return res
		}
	}

	if len(sn1) < len(sn2) {
		return 1
	}
	if len(sn1) > len(sn2) {
		return 0
	}

	return 2
}

func isSlice(v interface{}) bool {
	return reflect.TypeOf(v).Kind() == reflect.Slice
}

func min(n1 int, n2 int) int {
	if n1 < n2 {
		return n1
	}

	return n2
}
