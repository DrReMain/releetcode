package leetcode

func twoSum(nums []int, target int) []int {
	hashtable := map[int]int{}
	for idx, v := range nums {
		if v, ok := hashtable[target-v]; ok {
			return []int{v, idx}
		}
		hashtable[v] = idx
	}
	return nil
}
