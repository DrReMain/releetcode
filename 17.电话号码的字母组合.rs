/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return Vec::new();
        }

        let m: HashMap<char, Vec<String>> = vec![
            (
                '2',
                vec!["a", "b", "c"].iter().map(|s| s.to_string()).collect(),
            ),
            (
                '3',
                vec!["d", "e", "f"].iter().map(|s| s.to_string()).collect(),
            ),
            (
                '4',
                vec!["g", "h", "i"].iter().map(|s| s.to_string()).collect(),
            ),
            (
                '5',
                vec!["j", "k", "l"].iter().map(|s| s.to_string()).collect(),
            ),
            (
                '6',
                vec!["m", "n", "o"].iter().map(|s| s.to_string()).collect(),
            ),
            (
                '7',
                vec!["p", "q", "r", "s"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ),
            (
                '8',
                vec!["t", "u", "v"].iter().map(|s| s.to_string()).collect(),
            ),
            (
                '9',
                vec!["w", "x", "y", "z"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ),
        ]
        .into_iter()
        .collect();
        let digits = digits.chars().collect::<Vec<char>>();
        let mut ret: Vec<String> = Vec::new();
        Self::letter_combinations_recur(&m, ret.as_mut(), &digits, Vec::new().as_mut(), 0);
        ret
    }
    pub fn letter_combinations_recur(
        m: &HashMap<char, Vec<String>>,
        ret: &mut Vec<String>,
        digits: &Vec<char>,
        s: &mut Vec<String>,
        idx: usize,
    ) {
        if s.len() == digits.len() {
            ret.push(s.clone().join(""));
            return;
        }
        if idx < digits.len() {
            let char_list = &m[&digits[idx]];
            for item in char_list.iter() {
                let mut ns = s.clone();
                ns.push(item.clone());
                Self::letter_combinations_recur(m, ret, digits, ns.as_mut(), idx + 1);
            }
        }
    }
}
// @lc code=end

