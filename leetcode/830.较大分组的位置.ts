/*
 * @lc app=leetcode.cn id=830 lang=typescript
 *
 * [830] 较大分组的位置
 */

// @lc code=start
function largeGroupPositions(s: string): number[][] {
    let ret: number[][] = [];
    let start = 0;
    while (start < s.length) {
        let end = start + 1;
        for (;end < s.length && s[start] === s[end]; end++) {}
        if (end - start >= 3) ret.push([start, end-1]);
        start = end;
    }
    return ret;
};
// @lc code=end

