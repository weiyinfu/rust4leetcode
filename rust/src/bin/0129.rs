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

fn go(ans: &mut i32, root: &Option<Rc<RefCell<TreeNode>>>, value: i32) {
    if root.is_none() {
        return;
    }
    let node = root.as_deref().unwrap().borrow();
    if node.left.is_none() && node.right.is_none() {
        *ans += value * 10 + node.val;
        return;
    }
    go(ans, &node.left, value * 10 + node.val);
    go(ans, &node.right, value * 10 + node.val);
}

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut ans = 0;
        go(&mut ans, &root, 0);
        ans
    }
}

fn main() {
    println!("good");
}