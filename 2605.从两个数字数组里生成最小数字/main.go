package leetcode

import "math"

func minNumber(nums1 []int, nums2 []int) int {
	min1 := math.Inf(1)
	min2 := math.Inf(1)
	minc := math.Inf(1)
	m := make(map[int]bool)

	for _, item := range nums1 {
		min1 = math.Min(min1, float64(item))

		m[item] = true
	}

	for _, item := range nums2 {
		min2 = math.Min(min2, float64(item))

		if _, ok := m[item]; ok {
			minc = math.Min(minc, float64(item))
		}
	}

	if minc < math.Inf(1) {
		return int(minc)
	}

	if min1 < min2 {
		return int(min1*10 + min2)
	} else {
		return int(min2*10 + min1)
	}
}
