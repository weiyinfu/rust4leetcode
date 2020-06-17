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
use std::cmp::max;

fn go(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
    if root.is_none() {
        return (0, true);
    }
    let node = root.as_deref().unwrap().borrow();
    let left = go(&node.left);
    let right = go(&node.right);
    if !(left.1 && right.1) {
        return (0, false);
    }
    let mut dif = left.0 - right.0;
    if dif < 0 {
        dif = -dif;
    }
    if dif > 1 {
        return (0, false);
    }
    (max(left.0, right.0) + 1, true)
}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (h, is) = go(&root);
        is
    }
}

fn main() {
    println!("good");
}