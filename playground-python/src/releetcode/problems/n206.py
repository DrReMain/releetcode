from typing import Optional
from ..data_structures.list_node import ListNode

class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        prev = None
        node = head
        while node is not None:
            tmp = node.next
            node.next = prev
            prev = node
            node = tmp
        return prev

