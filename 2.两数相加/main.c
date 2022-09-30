#include <stdio.h>
#include <stdlib.h>

//  Definition for singly-linked list.
struct ListNode
{
    int val;
    struct ListNode *next;
};

struct ListNode *addTwoNumbers(struct ListNode *l1, struct ListNode *l2)
{
    struct ListNode *head = NULL, *tail = NULL;
    int c = 0;

    while (l1 || l2)
    {
        int sum = c + (l1 ? l1->val : 0) + (l2 ? l2->val : 0);

        if (!head)
        {
            head = tail = malloc(sizeof(struct ListNode));
            tail->val = sum % 10;
            tail->next = NULL;
        }
        else
        {
            tail->next = malloc(sizeof(struct ListNode));
            tail->next->val = sum % 10;
            tail->next->next = NULL;
            tail = tail->next;
        }

        c = sum / 10;
        l1 = l1 ? l1->next : NULL;
        l2 = l2 ? l2->next : NULL;
    }

    if (c > 0)
    {
        tail->next = malloc(sizeof(struct ListNode));
        tail->next->val = c;
        tail->next->next = NULL;
    }

    return head;
}