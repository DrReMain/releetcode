package n1

func twoSum(nums []int, target int) []int {
	dict := make(map[int]int, len(nums))

	for i, n := range nums {
		if idx, ok := dict[n]; ok {
			return []int{idx, i}
		}
		dict[target-n] = i
	}

	panic("")
}
