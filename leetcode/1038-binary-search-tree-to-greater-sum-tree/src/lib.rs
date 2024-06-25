// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

type Tree = Rc<RefCell<TreeNode>>;
pub fn bst_to_gst(mut root: Option<Tree>) -> Option<Tree> {
    fn dfs(root: Option<&mut Tree>, parent_sum: i32) -> i32 {
        if let Some(node) = root {
            let mut value = node.borrow().val;
            let right_child_sum = dfs(node.borrow_mut().right.as_mut(), parent_sum);
            value += parent_sum + right_child_sum;
            let left_child_sum = dfs(node.borrow_mut().left.as_mut(), value);
            std::mem::swap(&mut node.borrow_mut().val, &mut value);
            value + right_child_sum + left_child_sum
        } else {
            0
        }
    }

    dfs(root.as_mut(), 0);
    root
}
