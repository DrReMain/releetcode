/*
 * @lc app=leetcode.cn id=383 lang=golang
 *
 * [383] 赎金信
 */

// @lc code=start
func canConstruct(ransomNote string, magazine string) bool {
	if len(ransomNote) > len(magazine) {
		return false
	}
	list := [26]int{}
	for _, c := range magazine {
		list[c-'a']++
	}
	for _, c := range ransomNote {
		if list[c-'a'] > 0 {
			list[c-'a']--
		} else {
			return false
		}
	}
	return true
}
// @lc code=end

