/*
 * @lc app=leetcode.cn id=2807 lang=golang
 *
 * [2807] 在链表中插入最大公约数
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func insertGreatestCommonDivisors(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}
	node := head
	for node != nil && node.Next != nil {
		node.Next = &ListNode{gcd(node.Val, node.Next.Val), node.Next}
		node = node.Next.Next
	}
	return head
}
func gcd(a, b int) int {
	if b == 0 {
		return a
	}
	return gcd(b, a % b)
}
// @lc code=end

