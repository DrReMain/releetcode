/*
 * @lc app=leetcode.cn id=7 lang=typescript
 *
 * [7] 整数反转
 */

// @lc code=start
function reverse(x: number): number {
    let flag: boolean;
    let origin: number;
    let result = 0;
    if (x < 0) {
        flag = true;
        origin = x * -1;
    } else {
        flag = false;
        origin = x;
    }

    while (origin > 0) {
        result = result * 10 + origin % 10;
        if (result > 2 ** 31 - 1) return 0;
        origin = Math.floor(origin / 10);
    }

    return flag ? -result : result;
};
// @lc code=end

