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

fn go(pre: &[i32], mid: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if pre.len() == 0 {
        return None;
    }
    let mut ind = 0;
    while ind < mid.len() {
        if mid[ind] == pre[0] {
            break;
        }
        ind += 1;
    }
    let left = go(&pre[1..ind + 1], &mid[0..ind]);
    let right = go(&pre[ind + 1..], &mid[ind + 1..]);
    let mut root = TreeNode::new(pre[0]);
    root.left = left;
    root.right = right;
    Some(Rc::new(RefCell::new(root)))
}

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Index;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        go(&preorder, &inorder)
    }
}

fn main() {
    println!("haha");
}