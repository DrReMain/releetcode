#ifndef LIST_NODE_H
#define LIST_NODE_H

#include <stdlib.h>

struct ListNode {
    int val;
    struct ListNode *next;
};

struct ListNode* create_node(int val);
struct ListNode* from_array(int* arr, int size);
void free_list(struct ListNode* head);
int lists_equal(struct ListNode* l1, struct ListNode* l2);

#endif
