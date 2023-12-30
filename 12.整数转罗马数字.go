/*
 * @lc app=leetcode.cn id=12 lang=golang
 *
 * [12] 整数转罗马数字
 */

// @lc code=start
func intToRoman(num int) string {
	type tuple struct { value int; char string }
	tupleList := []tuple{
		tuple{1000, "M"},
		tuple{900, "CM"},
		tuple{500, "D"},
		tuple{400, "CD"},
		tuple{100, "C"},
		tuple{90, "XC"},
		tuple{50, "L"},
		tuple{40, "XL"},
		tuple{10, "X"},
		tuple{9, "IX"},
		tuple{5, "V"},
		tuple{4, "IV"},
		tuple{1, "I"},
	}
	
	ret := []string{};
	for _, t := range tupleList {
		for num >= t.value {
			num -= t.value
			ret = append(ret, t.char)
		}
		if num == 0 {
			break
		}
	}
	return strings.Join(ret, "")
}
// @lc code=end

