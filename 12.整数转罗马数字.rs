/*
 * @lc app=leetcode.cn id=12 lang=rust
 *
 * [12] 整数转罗马数字
 */

// @lc code=start
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let tupleList = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        let mut ret: Vec<String> = vec![];
        for (value, symble) in tupleList.into_iter() {
            while num >= value {
                num -= value;
                ret.push(symble.to_string());
            }
            if num == 0 {
                break;
            }
        }
        ret.join("")
    }
}
// @lc code=end

