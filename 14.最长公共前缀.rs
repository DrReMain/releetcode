/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] 最长公共前缀
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let mut strs = strs;
        strs.sort();
        let (start, end) = (&strs[0], &strs[strs.len()-1]);
        let mut i = 0;

        while i < start.len() && i < end.len() && 
            start.chars().nth(i) == end.chars().nth(i) {
                i += 1;
        }

        start[0..i].to_string()
    }
}
// @lc code=end

