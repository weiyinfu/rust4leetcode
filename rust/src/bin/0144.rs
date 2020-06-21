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

fn go(a: &mut Vec<i32>, root: &Option<Rc<RefCell<TreeNode>>>) {
    if root.is_none() {
        return;
    }
    let node = root.as_ref().unwrap().borrow();
    go(a, &node.left);
    go(a, &node.right);
    a.push(node.val);
}

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec!();
        go(&mut ans, &root);
        ans
    }
}