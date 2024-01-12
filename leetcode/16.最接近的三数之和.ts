/*
 * @lc app=leetcode.cn id=16 lang=typescript
 *
 * [16] 最接近的三数之和
 */

// @lc code=start
function threeSumClosest(nums: number[], target: number): number {
    nums.sort((a, b) => a - b)
    let closest = nums[0] + nums[1] + nums[2];

    for (let i = 0; i < nums.length - 2; ++i) {
        let [left, right] = [i + 1, nums.length - 1];
        while (left < right) {
            const sum = nums[i] + nums[left] + nums[right];
            if (sum < target) {
                left++;
            } else if (sum > target) {
                right--;
            } else {
                return sum;
            }

            if (Math.abs(target - sum) < Math.abs(target - closest))
                closest = sum;
        }
    }
    return closest;
};
// @lc code=end

