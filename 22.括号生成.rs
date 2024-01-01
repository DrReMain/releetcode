/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成
 */

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        fn backtrack(ret: &mut Vec<String>, n: &i32, s: &mut Vec<String>, left: i32, right: i32) {
            if s.len() as i32 == n * 2 {
                ret.push(s.clone().join(""));
                return;
            }
            if left < *n {
                let mut ns = s.clone();
                ns.push("(".to_string());
                backtrack(ret, n, &mut ns, left + 1, right);
            }
            if right < left {
                let mut ns = s.clone();
                ns.push(")".to_string());
                backtrack(ret, n, &mut ns, left, right + 1);
            }
        }
        backtrack(&mut ret, &n, Vec::new().as_mut(), 0, 0);
        ret
    }
}
// @lc code=end

