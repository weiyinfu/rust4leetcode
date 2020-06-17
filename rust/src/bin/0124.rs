// Definition for a binary tree node.
struct Solution;

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
use std::cmp::max;

fn go(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
    if root.is_none() {
        return 0;
    }
    let node = root.as_deref().unwrap().borrow();
    let left = go(&node.left, ans);
    let right = go(&node.right, ans);
    *ans = max(left + right + node.val, *ans);
    *ans = max(left + node.val, *ans);
    *ans = max(right + node.val, *ans);
    *ans = max(node.val, *ans);
    let sons = max(left, right);
    if sons < 0 {
        return node.val;
    } else {
        return sons + node.val;
    }
}

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = -1 << 30;
        go(&root, &mut ans);
        ans
    }
}

fn main() {
    println!("good");
}
