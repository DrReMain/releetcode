/*
 * @lc app=leetcode.cn id=525 lang=typescript
 *
 * [525] 连续数组
 */

// @lc code=start
function findMaxLength(nums: number[]): number {
    let max = 0;
    const m = new Map<number, number>();
    let counter = 0;
    m.set(counter, -1);

    for (let i = 0; i < nums.length; ++i) {
        if (nums[i] == 1) counter++;
        else counter--;

        if (m.has(counter)) {
            const prevIndex = m.get(counter) || 0;
            max = Math.max(max, i - prevIndex);
        } else {
            m.set(counter, i);
        }
    }

    return max;
};
// @lc code=end

