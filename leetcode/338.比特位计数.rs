/*
 * @lc app=leetcode.cn id=338 lang=rust
 *
 * [338] 比特位计数
 */

// @lc code=start
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![0; n as usize + 1];
        for i in 0..=n {
            let (mut remain, mut count) = (i, 0);
            while remain > 0 {
                if remain % 2 != 0 {
                    count += 1;
                }
                remain >>= 1;
            }
            ans[i as usize] = count;
        }
        ans
    }
}
// @lc code=end

