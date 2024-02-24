/*
 * @lc app=leetcode.cn id=1410 lang=typescript
 *
 * [1410] HTML 实体解析器
 */

// @lc code=start
function entityParser(text: string): string {
    // const m = {
    //     "quot": '"',
    //     "apos": "'",
    //     "amp": "&",
    //     "gt": ">",
    //     "lt": "<",
    //     "frasl": "/",
    // };

    // return text.replace(/&(quot|apos|amp|gt|lt|frasl);/g, (_, groupd) => m[groupd])


    const m = {
        "&quot;": '"',
        "&apos;": "'",
        "&amp;": "&",
        "&gt;": ">",
        "&lt;": "<",
        "&frasl;": "/",
    };

    const length = text.length;
    const ret: string[] = [];
    let i = 0;
    while (i < length) {
        let isEntity = false;
        if (text[i] === '&') {
            for (const [key, value] of Object.entries(m)) {
                if (text.slice(i, i + key.length) === key) {
                    ret.push(value);
                    isEntity = true;
                    i += key.length;
                    break;
                }
            }
        }
        if (!isEntity) {
            ret.push(text[i]);
            i++;
        }
    }
    return ret.join("");
};
// @lc code=end

