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

pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut childs = std::collections::HashSet::new();
    let mut val: std::collections::HashMap<i32, Rc<RefCell<TreeNode>>> = std::collections::HashMap::new();

    for d in descriptions {
        let (parent, child, left) = (d[0], d[1], d[2]);
        val.entry(parent).or_insert(Rc::new(RefCell::new(TreeNode::new(parent))));
        val.entry(child).or_insert(Rc::new(RefCell::new(TreeNode::new(child))));
        childs.insert(child);

        let parent_node = val.get(&parent).unwrap().clone();
        let child_node = val.get(&child).cloned();
        if left == 1 {
            (*parent_node).borrow_mut().left = child_node;
        } else {
            (*parent_node).borrow_mut().right = child_node
        }
    }

    let keys: std::collections::HashSet<_> = val.keys().cloned().collect();
    let root = keys.difference(&childs).next().unwrap();
    val.get(&root).cloned()
}
