/*
 * @lc app=leetcode.cn id=2706 lang=rust
 *
 * [2706] 购买两块巧克力
 */

// @lc code=start
impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        if prices.len() < 2 {
            return money;
        }

        let (mut min1, mut min2) = (i32::MAX, i32::MAX);
        for &p in prices.iter() {
            if p < min1 {
                (min1, min2) = (p, min1);
            } else if p < min2 {
                min2 = p;
            }
        }
        let remain = money - min1 - min2;
        if remain < 0 {
            money
        } else {
            remain
        }
    }
}
// @lc code=end
