#include "datastructure/list_node.h"
#include <stdio.h>

struct ListNode* create_node(int val) {
    struct ListNode* node = (struct ListNode*)malloc(sizeof(struct ListNode));
    node->val = val;
    node->next = NULL;
    return node;
}

struct ListNode* from_array(int* arr, int size) {
    if (size == 0) return NULL;
    struct ListNode* head = create_node(arr[0]);
    struct ListNode* curr = head;
    for (int i = 1; i < size; i++) {
        curr->next = create_node(arr[i]);
        curr = curr->next;
    }
    return head;
}

void free_list(struct ListNode* head) {
    struct ListNode* curr = head;
    while (curr) {
        struct ListNode* next = curr->next;
        free(curr);
        curr = next;
    }
}

int lists_equal(struct ListNode* l1, struct ListNode* l2) {
    while (l1 && l2) {
        if (l1->val != l2->val) return 0;
        l1 = l1->next;
        l2 = l2->next;
    }
    return l1 == NULL && l2 == NULL;
}
