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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
    let mut ans = 0;
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, distance: i32, ans: &mut i32) -> Vec<i32> {
        if node.is_some() {
            let node = node.as_ref().unwrap().borrow();
            let left = dfs(&node.left, distance, ans);
            let right = dfs(&node.right, distance, ans);
            let mut leafs = vec![0; distance as usize + 1];

            for (i, &l) in left.iter().enumerate() {
                for (j, &r) in right.iter().enumerate() {
                    if i + j + 2 <= distance as usize {
                        *ans += l * r;
                    }
                }
            }

            for i in 0..left.len() {
                if i+1 < leafs.len() {
                    leafs[i+1] += left[i];
                }
            }
            for i in 0..right.len() {
                if i+1 < leafs.len() {
                    leafs[i+1] += right[i];
                }
            }
            if node.left.is_none() && node.right.is_none() {
                leafs[0] = 1;
            }

            leafs
        } else {
            vec![]
        }
    }
    dfs(&root, distance, &mut ans);
    ans
}
