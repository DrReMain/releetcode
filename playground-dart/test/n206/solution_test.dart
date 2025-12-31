import 'package:test/test.dart';
import 'package:playground_dart/datastructure/list_node.dart';
import 'package:playground_dart/n206/solution.dart';

void main() {
  group('n206 Reverse List', () {
    final solution = Solution();

    test('Example 1', () {
      final input = ListNode.fromList([1, 2, 3, 4, 5]);
      final result = solution.reverseList(input);
      expect(result?.toList(), equals([5, 4, 3, 2, 1]));
    });

    test('Example 2', () {
      final input = ListNode.fromList([1, 2]);
      final result = solution.reverseList(input);
      expect(result?.toList(), equals([2, 1]));
    });

    test('Example 3', () {
      final input = ListNode.fromList([]);
      final result = solution.reverseList(input);
      expect(result, isNull);
    });
  });
}
