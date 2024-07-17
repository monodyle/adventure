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

pub fn del_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    to_delete: Vec<i32>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut ans = Vec::<Option<Rc<RefCell<TreeNode>>>>::new();
    if root.is_none() { return ans }

    fn collect(root: &mut Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>, s: &std::collections::HashSet<i32>) {
        let mut node = root.as_ref().unwrap().borrow_mut();
        if node.left.is_some() {
            collect(&mut node.left, ans, s);
            let val = node.left.as_ref().unwrap().borrow().val;
            if s.contains(&node.val) && !s.contains(&val) {
                ans.push(node.left.take());
            }
            if s.contains(&val) {
                node.left = None;
            }
        }
        if node.right.is_some() {
            collect(&mut node.right, ans, s);
            let val = node.right.as_ref().unwrap().borrow().val;
            if s.contains(&node.val) && !s.contains(&val) {
                ans.push(node.right.take());
            }
            if s.contains(&val) {
                node.right = None
            }
        }
    }

    let mut s = std::collections::HashSet::new();
    for a in to_delete {
        s.insert(a);
    }

    let mut root = root;
    collect(&mut root, &mut ans, &s);

    let val = root.as_ref().unwrap().borrow().val;
    if !s.contains(&val) {
        ans.push(root);
    }

    ans
}
