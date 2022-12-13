package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"reflect"
	"strings"
)

func main() {
	p1()
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

		if isOrdered(l1, l2) {
			fmt.Println(i + 1)
			sum += i
		}
	}

	fmt.Printf("Summed indizes: %d", sum)
}

func isOrdered(l1 []interface{}, l2 []interface{}, n int) bool {
	is_ordered := true

	if len(l1) == 0 || len(l2) == 0 {
		return true
	}

	for i := range l1 {
		if i > len(l2)-1 {
			return true
		}
		sl1 := isSlice(l1[i])
		sl2 := isSlice(l2[i])
		if sl1 && sl2 {
			res := isOrdered(l1[i].([]interface{}), l2[i].([]interface{}))
			if !res {
				return false
			}
			continue
		} else if sl1 && !sl2 {
			res := isOrdered(l1[i].([]interface{}), []interface{}{l2[i]})
			if !res {
				return false
			}
			continue
		} else if !sl1 && sl2 {
			res := isOrdered([]interface{}{l1[i]}, l2[i].([]interface{}))
			if !res {
				return false
			}
			continue
		}

		n1, ok := l1[i].(float64)
		if !ok {
			panic("n1 nan")
		}
		n2, ok := l2[i].(float64)
		if !ok {
			panic("n2 nan")
		}

		if n1 > n2 {
			return false
		}

	}

	if len(l1) < len(l2) {
		return true
	}

	return is_ordered
}

func isSlice(v interface{}) bool {
	return reflect.TypeOf(v).Kind() == reflect.Slice
}
