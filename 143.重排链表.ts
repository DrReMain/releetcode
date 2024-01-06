/*
 * @lc app=leetcode.cn id=143 lang=typescript
 *
 * [143] 重排链表
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

/**
 Do not return anything, modify head in-place instead.
 */
function reorderList(head: ListNode | null): void {
    if (!head || !head.next) return;

    let [middle, fast] = [head, head];
    while (fast && fast.next) {
        fast = fast.next.next;
        middle = middle.next;
    }
    
    // 反转
    let pre: ListNode | null = null;
    let node = middle;
    while (node) {
        let tmp = node.next;
        node.next = pre;
        pre = node;
        node = tmp;
    }

    // 合并
    let [cur1, cur2] = [head, pre];
    while (cur2!.next) {
        let first = cur1!.next;
        cur1.next = cur2!;
        cur1 = first;

        let second = cur2!.next;
        cur2!.next = cur1!;
        cur2 = second;
    }
};
// @lc code=end

