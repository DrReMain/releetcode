/*
 * @lc app=leetcode.cn id=1410 lang=typescript
 *
 * [1410] HTML 实体解析器
 */

// @lc code=start
function entityParser(text: string): string {
    const m = {
        "quot": '"',
        "apos": "'",
        "amp": "&",
        "gt": ">",
        "lt": "<",
        "frasl": "/",
    }

    return text.replace(/&(quot|apos|amp|gt|lt|frasl);/g, function (_, groupd) {
        return m[groupd]
    })
};
// @lc code=end

