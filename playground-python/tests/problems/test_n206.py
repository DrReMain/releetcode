import pytest
from releetcode.problems.n206 import Solution
from releetcode.data_structures.list_node import array_to_list, list_to_array

def test_reverse_list_1():
    sol = Solution()
    input_arr = [1, 2, 3, 4, 5]
    head = array_to_list(input_arr)
    result = sol.reverseList(head)
    assert list_to_array(result) == [5, 4, 3, 2, 1]

def test_reverse_list_2():
    sol = Solution()
    input_arr = [1, 2]
    head = array_to_list(input_arr)
    result = sol.reverseList(head)
    assert list_to_array(result) == [2, 1]

@pytest.mark.parametrize("input_arr, expected_arr", [
    ([1, 2, 3, 4, 5], [5, 4, 3, 2, 1]),
    ([1, 2], [2, 1]),
    ([], []),
    ([1], [1]),
])
def test_reverse_list_parametrized(input_arr, expected_arr):
    sol = Solution()
    head = array_to_list(input_arr)
    result = sol.reverseList(head)
    assert list_to_array(result) == expected_arr
