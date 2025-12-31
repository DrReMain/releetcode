#include "datastructure/list_node.h"
#include <stdio.h>
#include <time.h>

struct ListNode* reverseList(struct ListNode* head);

int main() {
    int arr[] = {1, 2, 3, 4, 5};
    struct ListNode* head = from_array(arr, 5);

    clock_t start = clock();
    int iterations = 1000000;
    for (int i = 0; i < iterations; i++) {
        struct ListNode* res = reverseList(head);
        // Note: In real benchmark, we'd need to reverse it back or use a copy
        // to keep input consistent, but for simple timing of this logic:
        head = res;
    }
    clock_t end = clock();

    double cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;
    printf("Time taken for %d iterations: %f seconds\n", iterations, cpu_time_used);
    printf("Average time per iteration: %f nanoseconds\n", (cpu_time_used / iterations) * 1e9);

    free_list(head);
    return 0;
}
