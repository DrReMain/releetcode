/*
 * @lc app=leetcode.cn id=2706 lang=typescript
 *
 * [2706] 购买两块巧克力
 */

// @lc code=start
function buyChoco(prices: number[], money: number): number {
    if (prices.length < 2) return money;

    let [min1, min2] = [+Infinity, +Infinity];
    for (const price of prices) {
        if (price < min1) {
            [min1, min2] = [price, min1];
        } else if (price < min2) {
            min2 = price;
        }
    }

    const remain = money - min1 - min2;

    return remain < 0 ? money : remain;
};
// @lc code=end

