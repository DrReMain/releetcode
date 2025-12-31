#include <gtest/gtest.h>
#include "datastructure/list_node.hpp"
#include "n206/solution.cpp"

TEST(N206, Example1) {
    Solution solution;
    ListNode* head = ListNode::fromVector({1, 2, 3, 4, 5});
    ListNode* result = solution.reverseList(head);
    std::vector<int> expected = {5, 4, 3, 2, 1};
    EXPECT_EQ(ListNode::toVector(result), expected);
    ListNode::freeList(result);
}

TEST(N206, Example2) {
    Solution solution;
    ListNode* head = ListNode::fromVector({1, 2});
    ListNode* result = solution.reverseList(head);
    std::vector<int> expected = {2, 1};
    EXPECT_EQ(ListNode::toVector(result), expected);
    ListNode::freeList(result);
}

TEST(N206, Example3) {
    Solution solution;
    ListNode* head = ListNode::fromVector({});
    ListNode* result = solution.reverseList(head);
    EXPECT_EQ(result, nullptr);
}
