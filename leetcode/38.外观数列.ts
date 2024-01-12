/*
 * @lc app=leetcode.cn id=38 lang=typescript
 *
 * [38] 外观数列
 */

// @lc code=start
function countAndSay(n: number): string {
    let ret = "1";
    for (let i = 2; i <= n; ++i) {
        let [p, start, tmp]: [number, number, string[]] = [0, 0, []];
        while (p < ret.length) {
            for (;p<ret.length && ret[p]===ret[start];++p) {}
            tmp.push(''+(p-start)+ret[start]);
            start = p;
        }
        ret = tmp.join("");
    }
    return ret;
};
// @lc code=end

