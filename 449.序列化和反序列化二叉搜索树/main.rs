use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(Default)]
struct Codec {
    idx: usize,
}

impl Codec {
    fn new() -> Self {
        return Codec::default();
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn pre_order(root: &Option<Rc<RefCell<TreeNode>>>) -> String {
            return if let Some(node) = root {
                let node = node.borrow();
                format!("{},{}{}", node.val, pre_order(&node.left), pre_order(&node.right))
            } else { "".to_string() };
        }
        pre_order(&root)
    }

    fn deserialize(&mut self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        use std::str::FromStr;
        if data.is_empty() { return None; };
        Self::dfs(self, &data.split(",").filter(|s| s.len() > 0).map(|s| i32::from_str(s).unwrap()).collect::<Vec<_>>(), i32::MIN, i32::MAX)
    }

    fn dfs(&mut self, arr: &Vec<i32>, minimum: i32, maximum: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if self.idx >= arr.len() { return None; }
        let curr = arr[self.idx];
        if curr < minimum || curr > maximum { return None; }
        self.idx += 1;
        Some(Rc::new(RefCell::new(TreeNode { val: curr, left: Self::dfs(self, arr, minimum, curr), right: Self::dfs(self, arr, curr, maximum) })))
    }
}

fn main() {
    println!("Hello, world!");
}
