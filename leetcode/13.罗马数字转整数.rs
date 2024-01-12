/*
 * @lc app=leetcode.cn id=13 lang=rust
 *
 * [13] 罗马数字转整数
 */

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;
        let s = s.chars().collect::<Vec<char>>();
        let m: HashMap<char, i32> = vec![
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .into_iter()
        .collect();
        let mut ret = 0;
        for i in 0..s.len() {
            let current_value = m[&s[i]];
            let next_value = if i + 1 < s.len() { m[&s[i + 1]] } else { 0 };
    
            if current_value < next_value {
                ret -= current_value;
            } else {
                ret += current_value;
            }
        }
        ret
    }
}
// @lc code=end
