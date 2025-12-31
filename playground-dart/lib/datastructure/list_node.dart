class ListNode {
  int val;
  ListNode? next;
  ListNode(this.val, [this.next]);

  static ListNode? fromList(List<int> list) {
    if (list.isEmpty) return null;
    ListNode head = ListNode(list[0]);
    ListNode curr = head;
    for (int i = 1; i < list.length; i++) {
      curr.next = ListNode(list[i]);
      curr = curr.next!;
    }
    return head;
  }

  List<int> toList() {
    List<int> res = [];
    ListNode? curr = this;
    while (curr != null) {
      res.add(curr.val);
      curr = curr.next;
    }
    return res;
  }
}
