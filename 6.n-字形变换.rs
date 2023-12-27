/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] N 字形变换
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }

        let mut ret: Vec<String> = vec![String::new(); num_rows as usize];
        let mut dir = 0i32;
        let mut flag = 1i32;
        for c in s.chars() {
            ret[dir as usize].push(c);

            let next_dir = dir + flag;
            if next_dir >= num_rows || next_dir < 0 {
                flag *= -1;
            }
            dir += flag;
        }
        ret.join("")
    }
}
// @lc code=end
