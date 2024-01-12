/*
 * @lc app=leetcode.cn id=46 lang=typescript
 *
 * [46] 全排列
 */

// @lc code=start
function permute(nums: number[]): number[][] {
    const ret: number[][] = [];
    function backtrack(path: number[]) {
        if (path.length === nums.length) {
            ret.push(path);
            return;
        }
        for (let i = 0; i < nums.length; ++i) {
            if (path.includes(nums[i])) continue;
            backtrack(path.concat(nums[i]));
        }
    }
    backtrack([]);
    return ret;
};
// @lc code=end

