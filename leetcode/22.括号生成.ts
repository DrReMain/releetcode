/*
 * @lc app=leetcode.cn id=22 lang=typescript
 *
 * [22] 括号生成
 */

// @lc code=start
function generateParenthesis(n: number): string[] {
    const ret: string[] = [];
    function backtrack(s: string[], left: number, right: number) {
        if (s.length === n * 2) {
            ret.push(s.join(""));
            return;
        }
        if (left < n) backtrack([...s, "("], left + 1, right);
        if (right < left) backtrack([...s, ")"], left, right + 1);
    }
    backtrack([], 0, 0);
    return ret;
};
// @lc code=end

