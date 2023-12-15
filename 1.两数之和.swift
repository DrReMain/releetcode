/*
 * @lc app=leetcode.cn id=1 lang=swift
 *
 * [1] 两数之和
 */

// @lc code=start
class Solution {
    func twoSum(_ nums: [Int], _ target: Int) -> [Int] {
        var map = [Int: Int]()
        for (i, n) in nums.enumerated() {
            if let v = map[target - n] {
                return [v, i]
            } else {
                map[n] = i
            }
        }
        return []
    }
}
// @lc code=end

