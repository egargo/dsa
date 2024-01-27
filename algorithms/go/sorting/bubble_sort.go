package main

import "fmt"

func sort(numbers []int) []int {
	for i := 0; i < len(numbers); i++ {
		for j := 0; j < len(numbers)-(i+1); j++ {
			left := numbers[j]
			right := numbers[j+1]

			if left > right {
				numbers[j] = right
				numbers[j+1] = left
			}
		}
	}

	return numbers
}

func main() {
	numbers := []int{6, 5, 0, 4, 3, 2, 1, -1, 9, 8, 7}
	fmt.Println(sort(numbers))
}
