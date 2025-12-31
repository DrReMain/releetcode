package com.releetcode.n206

import com.releetcode.datastructure.ListNode
import kotlin.test.Test
import kotlin.test.assertEquals

class SolutionTest {
    @Test
    fun testReverseList() {
        val solution = Solution()

        val input1 = ListNode.fromArray(intArrayOf(1, 2, 3, 4, 5))
        val output1 = solution.reverseList(input1)
        assertEquals(listOf(5, 4, 3, 2, 1), ListNode.toList(output1))

        val input2 = ListNode.fromArray(intArrayOf(1, 2))
        val output2 = solution.reverseList(input2)
        assertEquals(listOf(2, 1), ListNode.toList(output2))

        val input3 = ListNode.fromArray(intArrayOf())
        val output3 = solution.reverseList(input3)
        assertEquals(listOf(), ListNode.toList(output3))
    }
}
