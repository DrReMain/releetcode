/*
 * @lc app=leetcode.cn id=33 lang=typescript
 *
 * [33] 搜索旋转排序数组
 */

// @lc code=start
function search(nums: number[], target: number): number {
    if (nums.length === 0) return -1;
    
    let [l, r] = [0, nums.length - 1];
    while (l <= r) {
        const mid = (l+r) >> 1;
        if (nums[mid] == target) return mid;
        if (nums[0] <= nums[mid]) {
            if (nums[0] <= target && nums[mid] > target)
                r = mid - 1;
            else
                l = mid + 1;
        } else {
            if (nums[nums.length-1] >= target && nums[mid] < target)
                l = mid + 1;
            else
                r = mid - 1;
        }
    }
    return -1;
};
// @lc code=end

