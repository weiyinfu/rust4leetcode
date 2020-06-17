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
    {
        let mut root_node = root.as_deref().unwrap().borrow_mut();
        root_node.left = left;
        root_node.right = right;
        let mut left_node = root_node.left.as_deref().unwrap().borrow_mut();
        left_node.left = left_son.0;
        left_node.right = left_son.1;
    }
    let res = Solution::inorder_traversal(root.take());
    println!("{:?}", res);
}