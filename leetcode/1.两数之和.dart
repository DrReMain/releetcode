/*
 * @lc app=leetcode.cn id=1 lang=dart
 *
 * [1] 两数之和
 */

// @lc code=start
class Solution {
  List<int> twoSum(List<int> nums, int target) {
    var map = <int, int>{};
    for (int i = 0; i < nums.length; ++i) {
      var diff = target - nums[i];
      if (map[diff] != null) {
        return [map[diff]!, i];
      }
      map[nums[i]] = i;
    }

    return [0, 0];
  }
}
// @lc code=end

