/*
 * @lc app=leetcode.cn id=239 lang=typescript
 *
 * [239] 滑动窗口最大值
 */

// @lc code=start
function maxSlidingWindow(nums: number[], k: number): number[] {
    const length = nums.length;
    const q: number[] = [];
    for (let i = 0; i < k; ++i) {
        while (q.length && nums[i] >= nums[q[q.length - 1]])
            q.pop();
        q.push(i);
    }


    const ret = [nums[q[0]]];
    for (let i = k; i < length; ++i) {
        while (q.length && nums[i] >= nums[q[q.length - 1]])
            q.pop();
        q.push(i);

        while (q[0] <= i - k)
            q.shift();

        ret.push(nums[q[0]]);
    }


    return ret;
};
// @lc code=end

