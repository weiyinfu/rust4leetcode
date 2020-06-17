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

fn go(root: &mut Option<Rc<RefCell<TreeNode>>>, beg: i64, end: i64) -> bool {
    if root.is_none() {
        return true;
    }
    let node = root.as_deref().unwrap().borrow_mut();
    let val = node.val as i64;
    if !(val >= beg && val <= end) {
        return false;
    }
    let left_ok = go(&mut node.left, beg, val - 1);
    let right_ok = go(&mut node.right, val + 1, end);
    if !left_ok {}
    if !right_ok {}
    true
}

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        go(root, -1i64 << 32, 1i64 << 32);
    }
}