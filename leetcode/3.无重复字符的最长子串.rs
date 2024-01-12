/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let s = s.as_bytes();
        let mut m: HashMap<u8, bool> = HashMap::new();
        let mut ret = 0;
        let mut cur = 0;
        for (i, c) in s.iter().enumerate() {
            if i > 0 {
                m.insert(s[i - 1], false);
            }
            while cur < s.len() && !m.get(&s[cur]).map_or(false, |v| *v) {
                m.insert(s[cur], true);
                cur += 1;
            }
            ret = ret.max(cur - i);
        }
        ret as i32
    }
}
// @lc code=end

