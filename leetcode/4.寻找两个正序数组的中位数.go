/*
 * @lc app=leetcode.cn id=4 lang=golang
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	merge := append(nums1, nums2...)
	sort.Ints(merge)
	n := len(merge)

	if n % 2 == 0 {
		return float64(merge[(n>>1)-1]+merge[(n>>1)]) / 2.0
	} else {
		return float64(merge[(n>>1)])
	}
}

// @lc code=end

