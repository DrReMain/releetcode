/*
 * @lc app=leetcode.cn id=22 lang=typescript
 *
 * [22] 括号生成
 */

// @lc code=start
function generateParenthesis(n: number): string[] {
    const ret: string[] = [];
    function backtrack(path: string, left: number, right: number) {
        if (path.length === n * 2) {
            ret.push(path);
            return;
        }
        if (left < n) backtrack(path+"(", left + 1, right);
        if (right < left) backtrack(path+")", left, right + 1);
    }
    backtrack("", 0, 0);
    return ret;
};
// @lc code=end

