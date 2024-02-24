/*
 * @lc app=leetcode.cn id=25 lang=typescript
 *
 * [25] K 个一组翻转链表
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

function reverseKGroup(head: ListNode | null, k: number): ListNode | null {
    const dummy = new ListNode(0, head);
    let pre = dummy;

    while (head) {
        let tail = pre;
        for (let i = 0; i < k; ++i) {
            tail = tail.next;
            if (!tail) return dummy.next;
        }

        const next = tail.next;
        [head, tail] = reverse(head, tail);
        pre.next = head;
        tail.next = next;
        pre = tail;
        head = tail.next;
    }

    return dummy.next;
};
function reverse(head: ListNode, tail: ListNode): [ListNode, ListNode] {
    let prev = tail.next;
    let cur = head;
    while (prev !== tail) {
        const next = cur.next;
        cur.next = prev;
        prev = cur;
        cur = next;
    }
    return [tail, head];
}
// @lc code=end

