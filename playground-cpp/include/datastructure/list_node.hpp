#ifndef LIST_NODE_HPP
#define LIST_NODE_HPP

#include <vector>

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}

    static ListNode* fromVector(const std::vector<int>& nums);
    static std::vector<int> toVector(ListNode* head);
    static void freeList(ListNode* head);
};

#endif
