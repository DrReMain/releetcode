package DataStructure

type ListNodeVal interface {
	int | float64 | string
}

type ListNode[T ListNodeVal] struct {
	Val  T
	Next *ListNode[T]
}
