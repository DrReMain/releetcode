#!/usr/bin/python3
from typing import Optional

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        head = tail = None
        c = 0

        while l1 or l2:
            sum = c + (l1.val if l1 else 0) + (l2.val if l2 else 0)

            if not head:
                head = tail = ListNode(sum % 10)
            else: 
                tail.next = ListNode(sum % 10)
                tail = tail.next
        
            c = sum // 10
            if l1: l1 = l1.next
            if l2: l2 = l2.next

        if c > 0:
            tail.next = ListNode(c)


        return head
