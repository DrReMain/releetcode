/*
 * @lc app=leetcode.cn id=2807 lang=typescript
 *
 * [2807] 在链表中插入最大公约数
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

function insertGreatestCommonDivisors(head: ListNode | null): ListNode | null {
    if (!head || !head.next) return head;
    let node = head;
    while (node && node.next) {
        node.next = new ListNode(gcd(node.val, node.next.val), node.next);
        node = node.next.next;
    }
    return head;
};
function gcd(a: number, b: number): number {
    if (b === 0) return a;
    return gcd(b, a % b);
}
// @lc code=end

