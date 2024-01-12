/*
 * @lc app=leetcode.cn id=1 lang=kotlin
 *
 * [1] 两数之和
 */

// @lc code=start
class Solution {
    fun twoSum(nums: IntArray, target: Int): IntArray {
        val map: MutableMap<Int, Int> = mutableMapOf()
        for ((index, num) in nums.withIndex()) {
            val r = target - num
            val s = map.getOrDefault(r, -1)
            if (s != -1) {
                return intArrayOf(index, s)
            }
            map.put(num, index)
        }
        return intArrayOf(0, 0)
    }
}
// @lc code=end

