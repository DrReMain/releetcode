/*
 * @lc app=leetcode.cn id=383 lang=typescript
 *
 * [383] 赎金信
 */

// @lc code=start
function canConstruct(ransomNote: string, magazine: string): boolean {
    if (ransomNote.length > magazine.length) return false;
    const list: number[] = Array(26).fill(0);
    for (let i = 0; i < magazine.length; ++i) {
        list[magazine[i].charCodeAt(0) - 'a'.charCodeAt(0)]++;
    }
    for (let i = 0; i < ransomNote.length; ++i) {
        const idx = ransomNote[i].charCodeAt(0) - 'a'.charCodeAt(0);
        if (list[idx] > 0) list[idx]--;
        else return false;
    }
    return true;
};
// @lc code=end

