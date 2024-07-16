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

pub fn get_directions(
    root: Option<Rc<RefCell<TreeNode>>>,
    start_value: i32,
    dest_value: i32,
) -> String {
    let mut start_path = Vec::new();
    let mut dest_path = Vec::new();

    fn find_path(root: &Option<Rc<RefCell<TreeNode>>>, value: i32, path: &mut Vec<char>) -> bool {
        match root {
            None => false,
            Some(node) => {
                if node.borrow().val == value {
                    true
                } else {
                    path.push('L');
                    if find_path(&node.borrow().left, value, path) {
                        return true
                    }
                    path.pop();
                    path.push('R');
                    if find_path(&node.borrow().right, value, path) {
                        return true
                    }
                    path.pop();
                    false
                }
            }
        }
    }

    find_path(&root, start_value, &mut start_path);
    find_path(&root, dest_value, &mut dest_path);

    let mut cur = 0;
    for i in 0..(start_path.len()).min(dest_path.len()) {
        if start_path[i] == dest_path[i] {
            cur = i + 1;
        } else {
            break;
        }
    }

    start_path = start_path[cur..].to_vec();
    dest_path = dest_path[cur..].to_vec();

    let mut path = vec!['U'; start_path.len()];
    path.append(&mut dest_path);
    path.iter().collect()
}
