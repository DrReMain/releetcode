/*
 * @lc app=leetcode.cn id=215 lang=typescript
 *
 * [215] 数组中的第K个最大元素
 */

// @lc code=start
function findKthLargest(nums: number[], k: number): number {
    const n = nums.length;
    return quickSelect(nums, 0, n-1, n-k);
};
function quickSelect(nums: number[], l: number, r: number, k: number): number {
    if (l === r) return nums[k];
    let [p, i, j] = [nums[l], l-1, r+1];
    while (i < j) {
        for (i++;nums[i] < p; i++) {}
        for (j--;nums[j] > p; j--) {}
        
        if (i < j) [nums[i], nums[j]] = [nums[j], nums[i]];
    }
    if (k <= j) return quickSelect(nums, l, j, k);
    return quickSelect(nums, j+1, r, k);
}
// @lc code=end

