/*
 * @lc app=leetcode.cn id=40 lang=typescript
 *
 * [40] 组合总和 II
 */

// @lc code=start
function combinationSum2(candidates: number[], target: number): number[][] {
    let ret: number[][] = [];
    function backtrack(idx: number, item: number[], remain: number) {
        if (remain < 0) return;
        if (remain === 0) {
            ret.push([...item]);
            return;
        }

        for (let i = idx; i < candidates.length; ++i) {
            if (i > idx && candidates[i] === candidates[i-1]) continue;
            backtrack(i + 1, [...item, candidates[i]], remain - candidates[i]);
        }
    }
    candidates.sort((a, b) => a - b);
    backtrack(0, [], target);
    return ret;
};
// @lc code=end

