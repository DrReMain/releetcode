/*
 * @lc app=leetcode.cn id=39 lang=typescript
 *
 * [39] 组合总和
 */

// @lc code=start
function combinationSum(candidates: number[], target: number): number[][] {
    let ret: number[][] = [];
    function backtrack(item: number[], remain: number, idx: number) {
        if (idx === candidates.length) return;
        if (remain === 0) {
            ret.push(item);
            return;
        }
        backtrack(item, remain, idx+1);

        if (remain - candidates[idx] >= 0) {
            backtrack([...item, candidates[idx]], remain - candidates[idx], idx);
        }
    }
    backtrack([], target, 0);
    return ret;
};
// @lc code=end

