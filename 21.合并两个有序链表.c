/*
 * @lc app=leetcode.cn id=21 lang=c
 *
 * [21] 合并两个有序链表
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode *mergeTwoLists(struct ListNode *list1, struct ListNode *list2)
{
    struct ListNode *dummyHead = (struct ListNode *)malloc(sizeof(struct ListNode));
    dummyHead->next = NULL;
    struct ListNode *curr = dummyHead;
    while (list1 != NULL && list2 != NULL)
    {
        if (list1->val < list2->val)
        {
            curr->next = list1;
            list1 = list1->next;
        }
        else
        {
            curr->next = list2;
            list2 = list2->next;
        }
        curr = curr->next;
    }

    if (list1 != NULL)
        curr->next = list1;
    if (list2 != NULL)
        curr->next = list2;

    return dummyHead->next;
}
// @lc code=end
