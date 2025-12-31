import '../datastructure/list_node.dart';

class Solution {
  ListNode? reverseList(ListNode? head) {
    ListNode? prev;
    ListNode? curr = head;
    while (curr != null) {
      ListNode? nextTemp = curr.next;
      curr.next = prev;
      prev = curr;
      curr = nextTemp;
    }
    return prev;
  }
}
