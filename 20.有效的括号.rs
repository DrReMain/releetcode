/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::HashMap;
        let s = s.chars().collect::<Vec<char>>();
        let length = s.len();
        if length % 2 != 0 {
            return false;
        }
        let m: HashMap<_, _> = vec![(')', '('), ('}', '{'), (']', '[')]
            .into_iter()
            .collect();
        let mut tmp: Vec<char> = vec![];
        for c in s.iter() {
            match m.get(c) {
                Some(&val) if tmp.last() == Some(&val) => {
                    tmp.pop();
                }
                _ => {
                    tmp.push(*c);
                }
            }
        }
        tmp.is_empty()
    }
}
// @lc code=end
