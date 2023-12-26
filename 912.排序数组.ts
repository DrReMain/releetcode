/*
 * @lc app=leetcode.cn id=912 lang=typescript
 *
 * [912] 排序数组
 */

// @lc code=start
function sortArray(nums: number[]): number[] {
    if (nums.length !== 0)
        quickSort(nums, 0, nums.length - 1);
    return nums;
};
function quickSort(nums: number[], start: number, end: number) {
    if (start >= end) return;
    const mid = partition(nums, start, end);
    quickSort(nums, start, mid - 1);
    quickSort(nums, mid + 1, end);
}
function partition(nums: number[], start: number, end: number): number {
    const pivotIndex = Math.floor(Math.random() * (end - start + 1)) + start;
    const pivot = nums[pivotIndex];
    [nums[start], nums[pivotIndex]] = [nums[pivotIndex], nums[start]];

    let [left, right] = [start + 1, end];
    while (left < right) {
        while (left < right && nums[left] <= pivot) left++;
        while (left < right && nums[right] >= pivot) right--;
        if (left < right) {
            [nums[left], nums[right]] = [nums[right], nums[left]];
            left++;
            right--;
        }
    }
    if (left === right && nums[right] > pivot) right--;
    if (right !== start)
        [nums[start], nums[right]] = [nums[right], nums[start]];

    return right;
}
// @lc code=end

