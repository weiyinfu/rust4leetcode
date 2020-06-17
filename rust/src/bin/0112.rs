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

use std::rc::Rc;
use std::cell::RefCell;

fn go(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
    if root.is_none() {
        return sum == 0;
    }
    let node = root.as_deref().unwrap().borrow();
    if node.left.is_none() && node.right.is_none() {
        if sum == node.val {
            return true;
        } else {
            return false;
        }
    }
    if node.left.is_some() {
        let x = go(&node.left, sum - node.val);
        if x {
            return true;
        }
    }
    if node.right.is_some() {
        let x = go(&node.right, sum - node.val);
        if x {
            return true;
        }
    }
    false
}

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        go(&root, sum)
    }
}