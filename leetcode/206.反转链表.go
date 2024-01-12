/*
 * @lc app=leetcode.cn id=206 lang=golang
 *
 * [206] 反转链表
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func reverseList(head *ListNode) *ListNode {
	var pre *ListNode
	node := head
	for node != nil {
		tmp := node.Next
		node.Next = pre
		pre = node
		node = tmp
	}
	return pre;
}
// @lc code=end

