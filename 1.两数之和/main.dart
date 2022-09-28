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
