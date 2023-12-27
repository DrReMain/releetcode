/*
 * @lc app=leetcode.cn id=2660 lang=typescript
 *
 * [2660] 保龄球游戏的获胜者
 */

// @lc code=start
function isWinner(player1: number[], player2: number[]): number {
    let [sum1, sum2] = [0, 0];
    for (let i = 0; i < player1.length; ++i) {
        if (i === 0) {
            sum1 += player1[i];
            sum2 += player2[i];
        } else if (i === 1) {
            sum1 += player1[i - 1] === 10 ? 2 * player1[i] : player1[i];
            sum2 += player2[i - 1] === 10 ? 2 * player2[i] : player2[i];
        } else {
            sum1 += player1[i - 1] === 10 || player1[i - 2] === 10 ? 2 * player1[i] : player1[i];
            sum2 += player2[i - 1] === 10 || player2[i - 2] === 10 ? 2 * player2[i] : player2[i];
        }
    }
    return sum1 > sum2 ? 1 : sum1 < sum2 ? 2 : 0;
};
// @lc code=end

