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
        let mut m: HashMap<char, usize> = HashMap::new();
        let (mut ret, mut cur) = (0, 0);
        for (i, c) in s.iter().enumerate() {
            if let Some(idx) = m.get(c) {
                cur = cur.max(idx+1);
            }
            ret = ret.max(i+1-cur);
            m.insert(*c, i);
        }
        ret as i32
    }
}
// @lc code=end

