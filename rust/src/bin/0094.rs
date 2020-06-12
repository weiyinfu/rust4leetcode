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
use std::borrow::BorrowMut;

fn go(ans: &mut Vec<i32>, root: &Option<Rc<RefCell<TreeNode>>>) {
    if root.is_none() {
        return;
    }
    let node = root.as_deref().unwrap().borrow();
    go(ans, &node.left);
    ans.push(node.val);
    go(ans, &node.right);
}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec!();
        go(&mut ans, &root);
        ans
    }
}


fn main() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let mut left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let left_son = (
        Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        Some(Rc::new(RefCell::new(TreeNode::new(5)))),
    );
    left.as_mut().as_deref_mut().unwrap().get_mut().left = left_son.0;
    left.as_mut().as_deref_mut().unwrap().get_mut().right = left_son.1;
    root.as_mut().as_deref_mut().unwrap().get_mut().left = left;
    root.as_mut().as_deref_mut().unwrap().get_mut().right = right;
    let res = Solution::inorder_traversal(root);
    println!("{:?}", res);
}