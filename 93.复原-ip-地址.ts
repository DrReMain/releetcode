/*
 * @lc app=leetcode.cn id=93 lang=typescript
 *
 * [93] 复原 IP 地址
 */

// @lc code=start
function restoreIpAddresses(s: string): string[] {
    let ret: string[] = [];
    const count = 4;
    const segments = [0, 0, 0, 0];
    function dfs(s: string, idx: number, start: number) {
        if (idx === count) {
            if (start === s.length) ret.push(segments.join("."));
            return;
        }

        if (start === s.length) return;
        if (s[start] === '0') {
            segments[idx] = 0;
            dfs(s, idx + 1, start + 1);
            return;
        }

        let addr = 0;
        for (let end = start; end < s.length; ++end) {
            addr = addr * 10 + (s.charCodeAt(end) - 48);
            if (addr > 0 && addr <= 0xFF) {
                segments[idx] = addr;
                dfs(s, idx + 1, end + 1);
            } else break;
        }
    }
    dfs(s, 0, 0);
    return ret;
};
// @lc code=end

