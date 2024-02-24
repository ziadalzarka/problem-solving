package main

import "fmt"

func main() {
	collectedWater := trapRainWater([]int{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1})
	fmt.Printf("Collected %d blocks of water", collectedWater)
}

func trapRainWater(heights []int) int {

	count := len(heights)

	maxLefts := make([]int, count)
	maxRights := make([]int, count)

	currentMax := 0

	for i := 0; i < count; i++ {
		maxLefts[i] = currentMax

		if heights[i] > currentMax {
			currentMax = heights[i]
		}
	}

	currentMax = 0

	for i := count - 1; i > -1; i-- {
		maxRights[i] = currentMax

		if heights[i] > currentMax {
			currentMax = heights[i]
		}
	}

	collectedWater := 0

	for i := 0; i < count; i++ {
		collectedWater += max(0, min(maxLefts[i], maxRights[i])-heights[i])
	}

	return collectedWater
}
