package DataStructure

type ListNode struct {
	Val  int
	Next *ListNode
}

func NewListNode(val int) *ListNode {
	return &ListNode{Val: val, Next: nil}
}

func NewListNodeWithNode(val int, next *ListNode) *ListNode {
	return &ListNode{Val: val, Next: next}
}

func NewListNodeFromSlice(values []int) *ListNode {
	var head, current *ListNode
	for _, val := range values {
		node := NewListNode(val)
		if head == nil {
			head = node
			current = node
		} else {
			current.Next = node
			current = node
		}
	}
	return head
}
