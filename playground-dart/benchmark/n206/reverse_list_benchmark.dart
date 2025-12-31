import 'package:benchmark_harness/benchmark_harness.dart';
import 'package:playground_dart/datastructure/list_node.dart';
import 'package:playground_dart/n206/solution.dart';

class ReverseListBenchmark extends BenchmarkBase {
  ReverseListBenchmark() : super('ReverseList');

  final solution = Solution();
  ListNode? head;

  @override
  void setup() {
    head = ListNode.fromList([1, 2, 3, 4, 5]);
  }

  @override
  void run() {
    solution.reverseList(head);
  }
}

void main() {
  ReverseListBenchmark().report();
}
