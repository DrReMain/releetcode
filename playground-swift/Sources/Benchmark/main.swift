import Foundation
import n206
import DataStructure

let solution = Solution()
let head = ListNode.fromArray([1, 2, 3, 4, 5])

let iterations = 1000000
let start = CFAbsoluteTimeGetCurrent()

for _ in 0..<iterations {
    _ = solution.reverseList(head)
}

let end = CFAbsoluteTimeGetCurrent()
let timeUsed = end - start

print("Time taken for \(iterations) iterations: \(timeUsed) seconds")
print("Average time per iteration: \( (timeUsed / Double(iterations)) * 1_000_000_000 ) nanoseconds")
