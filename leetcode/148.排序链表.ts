/*
 * @lc app=leetcode.cn id=148 lang=typescript
 *
 * [148] 排序链表
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

function sortList(head: ListNode | null): ListNode | null {
    const list: ListNode[] = [];
    let node = head;
    while (node) {
        list.push(node);
        const tmp = node.next;
        node.next = null;
        node = tmp;
    }
    list.sort((a, b) => b.val - a.val);
    if (list.length === 0) return null;
    return list.reduce((big, small) => {
        if (!small) return big;
        small.next = big;
        return small;
    });
};
// @lc code=end

