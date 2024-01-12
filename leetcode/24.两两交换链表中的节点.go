/*
 * @lc app=leetcode.cn id=24 lang=golang
 *
 * [24] 两两交换链表中的节点
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func swapPairs(head *ListNode) *ListNode {
	dummyHead := &ListNode{0, head}
	node := dummyHead
	for node.Next != nil && node.Next.Next != nil {
		tmp := node.Next.Next.Next
		node.Next.Next.Next = node.Next
		node.Next = node.Next.Next
		node.Next.Next.Next = tmp
		node = node.Next.Next
	}
	return dummyHead.Next
}
// @lc code=end

