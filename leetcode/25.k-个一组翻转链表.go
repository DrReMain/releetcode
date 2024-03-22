/*
 * @lc app=leetcode.cn id=25 lang=golang
 *
 * [25] K 个一组翻转链表
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func reverseKGroup(head *ListNode, k int) *ListNode {
	dummy := &ListNode{0, head}
	node := dummy
	for head != nil {
		tail := node
		for i := 0; i < k; i++ {
			tail = tail.Next
			if tail == nil {
				return dummy.Next
			}
		}

		next := tail.Next
		head, tail = reverse(head, tail)
		tail.Next = next
		node.Next = head
		node = tail
		head = tail.Next
	}
	return dummy.Next
}
func reverse(head, tail *ListNode) (*ListNode, *ListNode) {
	prev := tail.Next
	node := head
	for prev != tail {
		next := node.Next
		node.Next = prev
		prev = node
		node = next
	}
	return tail, head
}
// @lc code=end

