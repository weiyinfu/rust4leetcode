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

fn go(x: &Option<Rc<RefCell<TreeNode>>>, y: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if x.is_none() && y.is_none() {
        return true;
    }
    if x.is_none() || y.is_none() {
        return false;
    }
    let xx = x.as_deref().unwrap().borrow();
    let yy = y.as_deref().unwrap().borrow();
    if xx.val != yy.val {
        return false;
    }
    let left = go(&xx.left, &yy.right);
    let right = go(&xx.right, &yy.left);
    if left && right {
        return true;
    }
    return false;
}

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        go(&root, &root)
    }
}

fn main() {
    println!("hello");
}