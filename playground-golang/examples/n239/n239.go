package n239

func maxSlidingWindow(nums []int, k int) []int {
	length := len(nums)
	q := []int{}
	for i := 0; i < k; i++ {
		for len(q) > 0 && nums[i] >= nums[q[len(q)-1]] {
			q = q[:len(q)-1]
		}
		q = append(q, i)
	}

	ret := []int{nums[q[0]]}
	for i := k; i < length; i++ {
		for len(q) > 0 && nums[i] >= nums[q[len(q)-1]] {
			q = q[:len(q)-1]
		}
		q = append(q, i)

		for q[0] <= i-k {
			q = q[1:]
		}
		ret = append(ret, nums[q[0]])
	}

	return ret
}
