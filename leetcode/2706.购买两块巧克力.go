/*
 * @lc app=leetcode.cn id=2706 lang=golang
 *
 * [2706] 购买两块巧克力
 */

// @lc code=start
func buyChoco(prices []int, money int) int {
	length := len(prices)
	if length < 2 {
		return money
	}

	min1, min2 := math.Inf(+1), math.Inf(+1)

	for _, p := range prices {
		pp := float64(p)
		if pp < min1 {
			min1, min2 = pp, min1
		} else if pp < min2 {
			min2 = pp
		}
	}

	remain := float64(money) - min1 - min2

	if remain < 0 {
		return money
	}
	return int(remain)
}

// @lc code=end

