use leetcode_common::data_structures::linked_list::ListNode;

pub struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut node = head;
        while let Some(mut n) = node {
            let tmp = n.next.take();
            n.next = prev;
            prev = Some(n);
            node = tmp;
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_reverse_list() {
        let input1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let expected1 = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        }));
        let result1 = Solution::reverse_list(input1);
        assert_eq!(result1, expected1);

        let input2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        let expected2 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        }));
        let result2 = Solution::reverse_list(input2);
        assert_eq!(result2, expected2);
    }

    #[bench]
    fn bench_reverse_list(b: &mut Bencher) {
        let input = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        b.iter(|| Solution::reverse_list(input.clone()));
    }
}
