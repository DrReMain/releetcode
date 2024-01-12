pub mod data_structures;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::*;
    use data_structures::binary_tree::TreeNode;
    use data_structures::linked_list::ListNode;

    #[test]
    fn test_binary_tree_creation() {
        let tree_node = TreeNode::new(42);
        assert_eq!(tree_node.val, 42);
        assert_eq!(tree_node.left, None);
        assert_eq!(tree_node.right, None);
    }

    #[test]
    fn test_linked_list_creation() {
        let list_node = ListNode::new(42);
        assert_eq!(list_node.val, 42);
        assert_eq!(list_node.next, None);
    }

    #[test]
    fn test_math_operations() {
        assert_eq!(utils::math_operations::add(2, 3), 5);
        assert_eq!(utils::math_operations::subtract(5, 3), 2);
        assert_eq!(utils::math_operations::multiply(2, 3), 6);
        assert_eq!(utils::math_operations::divide(6, 3), Some(2.0));
        assert_eq!(utils::math_operations::divide(1, 0), None);
    }
}