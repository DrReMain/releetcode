import XCTest
@testable import n206
import DataStructure

final class n206Tests: XCTestCase {
    func testExample1() {
        let solution = Solution()
        let head = ListNode.fromArray([1, 2, 3, 4, 5])
        let result = solution.reverseList(head)
        XCTAssertEqual(result?.toArray(), [5, 4, 3, 2, 1])
    }

    func testExample2() {
        let solution = Solution()
        let head = ListNode.fromArray([1, 2])
        let result = solution.reverseList(head)
        XCTAssertEqual(result?.toArray(), [2, 1])
    }

    func testExample3() {
        let solution = Solution()
        let head = ListNode.fromArray([])
        let result = solution.reverseList(head)
        XCTAssertNil(result)
    }
}
