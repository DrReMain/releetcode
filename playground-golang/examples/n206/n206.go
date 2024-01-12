package n206

import (
	"drremain.cn/playground-golang/DataStructure"
)

type ListNode = DataStructure.ListNode

func reverseList(head *ListNode) *ListNode {
	var prev *ListNode
	node := head
	for node != nil {
		tmp := node.Next
		node.Next = prev
		prev = node
		node = tmp
	}
	return prev
}
