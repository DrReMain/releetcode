import pytest
from releetcode.problems.n1 import Solution

def test_two_sum_benchmark(benchmark):
    sol = Solution()
    nums = list(range(10000))
    # We want to find the last two
    target = 9998 + 9999

    benchmark(sol.twoSum, nums, target)
