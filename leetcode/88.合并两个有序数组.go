/*
 * @lc app=leetcode.cn id=88 lang=golang
 *
 * [88] 合并两个有序数组
 */

// @lc code=start
func merge(nums1 []int, m int, nums2 []int, n int) {
	p, x, y := len(nums1) - 1, m - 1, n - 1
	for x >= 0 && y >= 0 {
		if nums1[x] < nums2[y] {
			nums1[p] = nums2[y]
			y--
		} else {
			nums1[p] = nums1[x]
			x--
		}
		p--
	}
	for i := 0; i <= y; i++ {
		nums1[i] = nums2[i]
	}
}

// @lc code=end

