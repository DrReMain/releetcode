package n25

import (
	"drremain.cn/playground-golang/DataStructure"
)

type ListNode = DataStructure.ListNode

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
	prev, node := tail.Next, head
	for prev != tail {
		next := node.Next
		node.Next = prev
		prev = node
		node = next
	}
	return tail, head
}
