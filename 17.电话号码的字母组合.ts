/*
 * @lc app=leetcode.cn id=17 lang=typescript
 *
 * [17] 电话号码的字母组合
 */

// @lc code=start
function letterCombinations(digits: string): string[] {
    if (digits.length === 0) return [];
    
    const m = new Map<string, string[]>([
        ["2", ["a", "b", "c"]],
        ["3", ["d", "e", "f"]],
        ["4", ["g", "h", "i"]],
        ["5", ["j", "k", "l"]],
        ["6", ["m", "n", "o"]],
        ["7", ["p", "q", "r", "s"]],
        ["8", ["t", "u", "v"]],
        ["9", ["w", "x", "y", "z"]],
    ]);
    const ret: string[] = [];
    function recur(s: string[], idx: number) {
        if (s.length === digits.length) {
            ret.push(s.join(""));
            return;
        }
        if (idx < digits.length) {
            const charList = m.get(digits[idx]);
            for (let i = 0; charList?.length && i < charList.length; ++i) {
                recur(s.concat(charList[i]), idx+1);
            }
        }
    }
    recur([], 0);
    return ret;
};
// @lc code=end

