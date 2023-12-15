/*
 * @lc app=leetcode.cn id=1 lang=csharp
 *
 * [1] 两数之和
 */

// @lc code=start
public class Solution
{
    public int[] TwoSum(int[] nums, int target)
    {
        Dictionary<int, int> dic = new();
        for (int i = 0; i < nums.Length; i++)
        {
            int imp = target - nums[i];
            if (dic.ContainsKey(imp) && dic[imp] != i)
            {
                return new int[] { i, dic[imp] };
            }
            if (!dic.ContainsKey(nums[i]))
            {
                dic.Add(nums[i], i);
            }
        }
        return new int[] { 0, 0 };
    }
}
// @lc code=end

