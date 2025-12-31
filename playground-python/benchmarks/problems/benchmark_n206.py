import pytest
from releetcode.problems.n206 import Solution
from releetcode.data_structures.list_node import array_to_list

def test_reverse_list_benchmark(benchmark):
    sol = Solution()
    head = array_to_list([1, 2, 3, 4, 5])
    benchmark(sol.reverseList, head)
