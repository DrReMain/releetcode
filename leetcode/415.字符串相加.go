/*
 * @lc app=leetcode.cn id=415 lang=golang
 *
 * [415] 字符串相加
 */

// @lc code=start
func addStrings(num1 string, num2 string) (ret string) {
	i, j, carry := len(num1)-1, len(num2)-1, 0

	for ; i >= 0 || j >= 0 || carry > 0; i, j = i-1, j-1 {
		n1, n2 := 0, 0
		if i >= 0 {
			n1 = int(num1[i] - '0')
		}
		if j >= 0 {
			n2 = int(num2[j] - '0')
		}
		sum := n1 + n2 + carry

		ret = string(sum%10 + '0') + ret
		carry = sum / 10
	}
	return
}

// @lc code=end

