/*
 * @lc app=leetcode.cn id=300 lang=golang
 *
 * [300] 最长递增子序列
 */

// @lc code=start
func lengthOfLIS(nums []int) int {
	tmp := []int{}
	for _, n := range nums {
		length := len(tmp)
		if length == 0 || n > tmp[length-1] {
			tmp = append(tmp, n)
			continue
		}

		left, right := 0, length-1
		p := right
		for left <= right {
			mid := (left + right) >> 1
			if tmp[mid] >= n {
				p = mid
				right = mid - 1
			} else {
				left = mid + 1
			}
		}
		tmp[p] = n
	}
	return len(tmp)
}

// @lc code=end

