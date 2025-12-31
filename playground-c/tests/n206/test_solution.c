#include "datastructure/list_node.h"
#include <stdio.h>
#include <assert.h>

struct ListNode* reverseList(struct ListNode* head);

void test_example1() {
    int arr[] = {1, 2, 3, 4, 5};
    struct ListNode* head = from_array(arr, 5);
    struct ListNode* result = reverseList(head);

    int expected_arr[] = {5, 4, 3, 2, 1};
    struct ListNode* expected = from_array(expected_arr, 5);

    assert(lists_equal(result, expected));
    printf("Test Example 1 Passed\n");

    free_list(result);
    free_list(expected);
}

void test_example2() {
    int arr[] = {1, 2};
    struct ListNode* head = from_array(arr, 2);
    struct ListNode* result = reverseList(head);

    int expected_arr[] = {2, 1};
    struct ListNode* expected = from_array(expected_arr, 2);

    assert(lists_equal(result, expected));
    printf("Test Example 2 Passed\n");

    free_list(result);
    free_list(expected);
}

void test_example3() {
    struct ListNode* head = NULL;
    struct ListNode* result = reverseList(head);
    assert(result == NULL);
    printf("Test Example 3 Passed\n");
}

int main() {
    test_example1();
    test_example2();
    test_example3();
    return 0;
}
