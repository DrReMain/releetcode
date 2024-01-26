/*
 * @lc app=leetcode.cn id=82 lang=typescript
 *
 * [82] 删除排序链表中的重复元素 II
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

function deleteDuplicates(head: ListNode | null): ListNode | null {
    if (!head) return head;

    const dummyHead = new ListNode(0, head);
    let cur = dummyHead;
    while (cur.next && cur.next.next) {
        if (cur.next.val === cur.next.next.val) {
            const n = cur.next.val;
            while (cur.next && cur.next.val === n)
                cur.next = cur.next.next;
        } else cur = cur.next;
    }

    return dummyHead.next;
};
// @lc code=end

