/*
 * @lc app=leetcode.cn id=8 lang=golang
 *
 * [8] 字符串转换整数 (atoi)
 */

// @lc code=start
func myAtoi(s string) (result int) {
    flag, i, n := 1, 0, len(s)
    for ; i < n && s[i] == ' '; i++ {}
    if i >= n {
        return 0
    }

    switch s[i] {
        case '+':
            i++
        case '-':
            i++
            flag = -1
    }

    for ; i < n; i++ {
        if s[i] < byte('0') || s[i] > byte('9') {
            break
        }
        result = result*10 + int(s[i]-'0')
        if flag*result < math.MinInt32 {
            return math.MinInt32
        }
        if flag*result > math.MaxInt32 {
            return math.MaxInt32
        }
    }
    return flag * result
}
// @lc code=end

