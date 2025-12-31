#include <benchmark/benchmark.h>
#include "datastructure/list_node.hpp"
#include "n206/solution.cpp"

static void BM_ReverseList(benchmark::State& state) {
    Solution solution;
    for (auto _ : state) {
        state.PauseTiming();
        ListNode* head = ListNode::fromVector({1, 2, 3, 4, 5});
        state.ResumeTiming();

        ListNode* result = solution.reverseList(head);

        state.PauseTiming();
        ListNode::freeList(result);
        state.ResumeTiming();
    }
}
BENCHMARK(BM_ReverseList);

BENCHMARK_MAIN();
