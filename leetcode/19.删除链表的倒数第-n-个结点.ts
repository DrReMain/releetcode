/*
 * @lc app=leetcode.cn id=19 lang=typescript
 *
 * [19] 删除链表的倒数第 N 个结点
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

function removeNthFromEnd(head: ListNode | null, n: number): ListNode | null {
    const dummyHead = new ListNode(0, head);
    let [cur, tail] = [dummyHead, dummyHead];
    for (let i = 1; i <= n; i++)
        tail = tail.next;
    while (tail) {
        tail = tail.next;
        if (tail) cur = cur.next;
    }
    cur.next = cur.next.next;
	return dummyHead.next; 
};
// @lc code=end

