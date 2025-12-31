package com.releetcode.n206

import com.releetcode.datastructure.ListNode
import org.openjdk.jmh.annotations.*
import java.util.concurrent.TimeUnit

@State(Scope.Benchmark)
@BenchmarkMode(Mode.AverageTime)
@OutputTimeUnit(TimeUnit.NANOSECONDS)
open class ReverseListBenchmark {
    private val solution = Solution()
    private var head: ListNode? = null

    @Setup
    fun setup() {
        head = ListNode.fromArray(intArrayOf(1, 2, 3, 4, 5))
    }

    @Benchmark
    fun benchmarkReverseList(): ListNode? {
        return solution.reverseList(head)
    }
}
