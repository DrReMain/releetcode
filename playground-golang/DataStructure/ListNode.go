package DataStructure

type listNodeVal interface {
	int | float64 | string
}

type ListNode[T listNodeVal] struct {
	Val  T
	Next *ListNode[T]
}
