/*
 * @lc app=leetcode.cn id=19 lang=golang
 *
 * [19] 删除链表的倒数第 N 个结点
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func removeNthFromEnd(head *ListNode, n int) *ListNode {
	dummyHead := &ListNode{0, head}
	cur, tail := dummyHead, dummyHead
	for i := 1; i <= n; i++ {
		tail = tail.Next
	}
	for tail != nil {
		tail = tail.Next
		if tail != nil {
			cur = cur.Next
		}
	}
	
	cur.Next = cur.Next.Next
	return dummyHead.Next
}
// @lc code=end

