package leetcode

type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1, l2 *ListNode) (head *ListNode) {
	var tail *ListNode
	c := 0

	for l1 != nil || l2 != nil {
		// l1和l2可能为nil，取l1和l2的Val需要一个初始值变量
		n1, n2 := 0, 0

		if l1 != nil {
			n1 = l1.Val
			l1 = l1.Next
		}

		if l2 != nil {
			n2 = l2.Val
			l2 = l2.Next
		}

		sum := c + n1 + n2
		sum, c = sum%10, sum/10

		if head == nil {
			tail = &ListNode{Val: sum}
			head = tail
		} else {
			tail.Next = &ListNode{Val: sum}
			tail = tail.Next
		}
	}

	if c > 0 {
		tail.Next = &ListNode{Val: c}
	}

	return
}
