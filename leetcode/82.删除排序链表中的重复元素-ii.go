/*
 * @lc app=leetcode.cn id=82 lang=golang
 *
 * [82] 删除排序链表中的重复元素 II
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func deleteDuplicates(head *ListNode) *ListNode {
	if head == nil {
		return head
	}

	dummy := &ListNode{0, head}
	node := dummy
	for node.Next != nil && node.Next.Next != nil {
		if node.Next.Val == node.Next.Next.Val {
			v := node.Next.Val
			for node.Next != nil && node.Next.Val == v {
				node.Next = node.Next.Next
			}
		} else {
			node = node.Next
		}
	}
	return dummy.Next
}
// @lc code=end

