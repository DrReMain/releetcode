/*
 * @lc app=leetcode.cn id=300 lang=typescript
 *
 * [300] 最长递增子序列
 */

// @lc code=start
function lengthOfLIS(nums: number[]): number {
    const tmp: number[] = [];
    for (let n of nums) {
        const length = tmp.length;
        if (length === 0 || n > tmp[length-1]) {
            tmp.push(n);
            continue;
        }

        let [left, right] = [0, length-1];
        let p = right;
        while (left <= right) {
            const mid = (left+right) >> 1;
            if (tmp[mid] >= n) {
                p = mid;
                right = mid-1;
            } else left = mid+1;
        }
        tmp[p] = n;
    }
    return tmp.length;
};
// @lc code=end

