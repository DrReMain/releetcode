package leetcode

type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	var result *ListNode
	s1, s2 := []int{}, []int{}
	cur := 0

	for l1 != nil {
		s1 = append(s1, l1.Val)
		l1 = l1.Next
	}
	for l2 != nil {
		s2 = append(s2, l2.Val)
		l2 = l2.Next
	}

	for len(s1) > 0 || len(s2) > 0 || cur != 0 {
		if len(s1) > 0 {
			cur += s1[len(s1)-1]
			s1 = s1[:len(s1)-1]
		}
		if len(s2) > 0 {
			cur += s2[len(s2)-1]
			s2 = s2[:len(s2)-1]
		}
		node := &ListNode{Val: cur % 10}
		cur /= 10
		node.Next = result
		result = node
	}

	return result
}
