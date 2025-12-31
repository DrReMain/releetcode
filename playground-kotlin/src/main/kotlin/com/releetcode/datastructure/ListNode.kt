package com.releetcode.datastructure

class ListNode(var `val`: Int) {
    var next: ListNode? = null

    companion object {
        fun fromArray(arr: IntArray): ListNode? {
            if (arr.isEmpty()) return null
            val head = ListNode(arr[0])
            var curr = head
            for (i in 1 until arr.size) {
                curr.next = ListNode(arr[i])
                curr = curr.next!!
            }
            return head
        }

        fun toList(head: ListNode?): List<Int> {
            val res = mutableListOf<Int>()
            var curr = head
            while (curr != null) {
                res.add(curr.`val`)
                curr = curr.next
            }
            return res
        }
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other !is ListNode) return false
        return `val` == other.`val` && next == other.next
    }

    override fun hashCode(): Int {
        var result = `val`
        result = 31 * result + (next?.hashCode() ?: 0)
        return result
    }
}
