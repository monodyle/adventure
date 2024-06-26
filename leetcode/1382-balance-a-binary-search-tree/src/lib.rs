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


struct Solution {}

type Tree = Rc<RefCell<TreeNode>>;
impl Solution {
    pub fn balance_bst(root: Option<Tree>) -> Option<Tree> {
        let mut sorted: Vec<i32> = Vec::with_capacity(50);
        Self::inorder(root, &mut sorted);
        let e = sorted.len();
        Self::construct(&mut sorted, 0, e as i32-1)
    }

    fn construct(sorted: &mut Vec<i32>, s: i32, e: i32) -> Option<Tree> {
        if s > e {
            return None;
        }
        let m = ((s + e) >> 1) as usize;
        let mut root = TreeNode::new(sorted[m]);
        root.left = Self::construct(sorted, s, m as i32 - 1);
        root.right = Self::construct(sorted, m as i32 + 1, e);
        Some(Rc::new(RefCell::new(root)))
    }

    fn inorder(root: Option<Tree>, sorted: &mut Vec<i32>) {
        if root.as_ref().is_none() {
            return;
        }
        let r = root.as_ref().unwrap().borrow();
        Self::inorder(r.left.clone(), sorted);
        sorted.push(r.val);
        Self::inorder(r.right.clone(), sorted);
    }
}
