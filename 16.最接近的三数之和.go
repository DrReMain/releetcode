/*
 * @lc app=leetcode.cn id=16 lang=golang
 *
 * [16] 最接近的三数之和
 */

// @lc code=start
func threeSumClosest(nums []int, target int) int {
	sort.Ints(nums)
	closest := nums[0] + nums[1] + nums[2]
	for i := 0; i < len(nums)-2; i++ {
		left, right := i+1, len(nums)-1
		for left < right {
			sum := nums[i] + nums[left] + nums[right]
			if sum < target {
				left++
			} else if sum > target {
				right--
			} else {
				return sum
			}

			if abs(target-sum) < abs(target-closest) {
				closest = sum
			}
		}
	}
	return closest
}
func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

// @lc code=end

