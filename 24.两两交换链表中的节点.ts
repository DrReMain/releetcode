/*
 * @lc app=leetcode.cn id=24 lang=typescript
 *
 * [24] 两两交换链表中的节点
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function swapPairs(head: ListNode | null): ListNode | null {
    const dummyHead = new ListNode(0, head);
    let node = dummyHead;
    while (node.next && node.next.next) {
        const tmp = node.next.next.next;
        node.next.next.next = node.next;
        node.next = node.next.next;
        node.next.next.next = tmp;
        node = node.next.next;
    }
    return dummyHead.next;
};
// @lc code=end

