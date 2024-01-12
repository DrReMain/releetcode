#
# @lc app=leetcode.cn id=53 lang=python3
#
# [53] 最大子数组和
#

# @lc code=start
class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        pre, ret = 0, nums[0]
        for v in nums:
            pre = max(pre+v, v)
            ret = max(ret, pre)
        return ret
# @lc code=end

