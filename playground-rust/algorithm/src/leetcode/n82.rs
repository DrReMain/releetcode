use leetcode_common::data_structures::linked_list::ListNode;

pub struct Solution;
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut prev_val = None;
        while let Some(mut node) = head {
            head = node.next.take();
            let node_val = node.val;

            if head.as_ref().map_or(true, |next| next.val != node_val)
                && prev_val.map_or(true, |val| val != node_val)
            {
                tail.next = Some(node);
                tail = tail.next.as_mut().unwrap();
            }
            prev_val = Some(node_val);
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_delete_duplicates1() {
        let head = ListNode::from(vec![1, 2, 3, 3, 4, 4, 5]);
        let expected = ListNode::from(vec![1, 2, 5]);
        assert_eq!(Solution::delete_duplicates(head), expected);
    }

    #[test]
    fn test_delete_duplicates2() {
        let head = ListNode::from(vec![1, 1, 1, 2, 3]);
        let expected = ListNode::from(vec![2, 3]);
        assert_eq!(Solution::delete_duplicates(head), expected);
    }

    #[bench]
    fn bench_delete_duplicates(b: &mut Bencher) {
        let head = ListNode::from(vec![1, 2, 3, 3, 4, 4, 5]);
        b.iter(|| Solution::delete_duplicates(head.clone()));
    }
}
