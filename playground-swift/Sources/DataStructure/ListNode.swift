public class ListNode {
    public var val: Int
    public var next: ListNode?
    public init() { self.val = 0; self.next = nil; }
    public init(_ val: Int) { self.val = val; self.next = nil; }
    public init(_ val: Int, _ next: ListNode?) { self.val = val; self.next = next; }

    public static func fromArray(_ array: [Int]) -> ListNode? {
        guard !array.isEmpty else { return nil }
        let head = ListNode(array[0])
        var curr = head
        for i in 1..<array.count {
            curr.next = ListNode(array[i])
            curr = curr.next!
        }
        return head
    }

    public func toArray() -> [Int] {
        var res = [Int]()
        var curr: ListNode? = self
        while curr != nil {
            res.append(curr!.val)
            curr = curr!.next
        }
        return res
    }
}
