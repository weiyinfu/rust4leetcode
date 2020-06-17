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

struct Solution;


use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::min;

fn go(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let node = root.as_deref().unwrap().borrow();
    let mut h = 0xffffff;
    if node.left.is_none() && node.right.is_none() {
        return 1;
    }
    if node.left.is_some() {
        let left = go(&node.left);
        h = min(left, h);
    }
    if node.right.is_some() {
        let right = go(&node.right);
        h = min(h, right);
    }
    return h + 1;
}

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        go(&root)
    }
}

