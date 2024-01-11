/*
 * @lc app=leetcode.cn id=38 lang=rust
 *
 * [38] 外观数列
 */

// @lc code=start
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut ret = vec!['1'];
        for i in 2..=n {
            let (mut p, mut start) = (0, 0);
            let mut tmp = Vec::new();
            while p < ret.len() {
                while p < ret.len() && ret[p] == ret[start] {
                    p += 1;
                }
                tmp.extend((p - start).to_string().chars().collect::<Vec<char>>());
                tmp.push(ret[start]);
                start = p;
            }
            ret = tmp;
        }
        ret.iter().collect()
    }
}
// @lc code=end
