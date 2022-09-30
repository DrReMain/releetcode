// Definition for singly-linked list.
class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

function addTwoNumbers(
  l1: ListNode | null,
  l2: ListNode | null
): ListNode | null {
  let head: ListNode | null = null;
  let tail: ListNode | null = null;
  let c = 0;

  while (l1 || l2) {
    // 每个节点的和 与 上一个进位（如果有） 相加
    // 根据这个和来处理结果的节点
    const sum = c + (l1?.val || 0) + (l2?.val || 0);

    if (!head) {
      // 首次循环初始化head和tail，为首次sum的余数
      head = tail = new ListNode(sum % 10);
    } else {
      // 后续循环，创建的节点为tail的next
      tail!.next = new ListNode(sum % 10);
      // 并将tail指向 它的next
      tail = tail!.next;
    }

    // 每次处理完结果的节点后，处理进位值
    c = Math.floor(sum / 10);

    // l1和l2指向它们各自的next，进行下次循环
    l1 = l1?.next || null;
    l2 = l2?.next || null;
  }

  // 如果还有进位值，则创建最后一个节点
  if (c) tail!.next = new ListNode(c);

  return head;
}
