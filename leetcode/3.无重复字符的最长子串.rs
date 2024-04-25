/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;

        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        if n < 2 {
            return n as i32;
        }
        let mut m = HashMap::<char, usize>::new();
        let (mut ret, mut p) = (0, 0);
        for (i, c) in s.iter().enumerate() {
            if let Some(idx) = m.get(c) {
                p = p.max(idx+1);
            }
            ret = ret.max(i+1-p);
            m.insert(*c, i);
        }
        ret as i32
    }
}
// @lc code=end

