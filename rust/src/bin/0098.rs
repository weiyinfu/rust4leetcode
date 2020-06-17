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
use std::ops::Deref;

fn check(node: &Option<Rc<RefCell<TreeNode>>>, beg: i64, end: i64) -> bool {
    if node.is_none() {
        return true;
    }
    let no = node.as_deref().unwrap().borrow();
    if !(no.val as i64 >= beg && no.val as i64 <= end) {
        return false;
    }
    check(&no.left, beg, no.val as i64 - 1) &&
        check(&no.right, no.val as i64 + 1, end)
}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        check(&root, -1i64 << 32, 1i64 << 32)
    }
}


fn main() {
    println!("h");
}