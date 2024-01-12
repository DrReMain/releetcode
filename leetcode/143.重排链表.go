/*
 * @lc app=leetcode.cn id=143 lang=golang
 *
 * [143] 重排链表
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func reorderList(head *ListNode)  {
	if head == nil || head.Next == nil {
		return
	}

	// 寻找中间节点
	middle, fast := head, head
	for fast != nil && fast.Next != nil {
		fast = fast.Next.Next
		middle = middle.Next
	}

	// 反转
	var pre *ListNode
	node := middle
	for node != nil {
		tmp := node.Next
		node.Next = pre
		pre = node
		node = tmp
	}

	// 合并
	cur1, cur2 := head, pre
	for cur2.Next != nil {
		tmp1 := cur1.Next
		cur1.Next = cur2
		cur1 = tmp1;

		tmp2 := cur2.Next
		cur2.Next = cur1
		cur2 = tmp2
	}
}
// @lc code=end

