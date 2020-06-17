// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

struct Solution;

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

fn go(mid: &[i32], post: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if mid.len() == 0 {
        return None;
    }
    let mut ind = 0;
    while ind < mid.len() {
        if mid[ind] == *post.last().unwrap() {
            break;
        }
        ind += 1;
    }
    let left = go(&mid[..ind], &post[..ind]);
    let right = go(&mid[ind + 1..], &post[ind..post.len() - 1]);
    let mut root = TreeNode::new(*post.last().unwrap());
    root.left = left;
    root.right = right;
    Some(Rc::new(RefCell::new(root)))
}

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        go(&inorder, &postorder)
    }
}


fn main() {
    let mid = vec![9, 3, 15, 20, 7];
    let post = vec![9, 15, 7, 20, 3];
    let ans = Solution::build_tree(mid, post);
    println!("good");
    println!("{:?}", ans);
}