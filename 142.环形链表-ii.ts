/*
 * @lc app=leetcode.cn id=142 lang=typescript
 *
 * [142] 环形链表 II
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

function detectCycle(head: ListNode | null): ListNode | null {
    if (!head || !head.next) return null;
    let [slow, fast, ret] = [head, head, head];
    while (fast && fast.next) {
        fast = fast.next.next;
        slow = slow.next;
        if (fast === slow) {
            while (ret !== slow) {
                slow = slow.next;
                ret = ret.next;
            }
            return ret;
        }
    }

    return null;
};
// @lc code=end

