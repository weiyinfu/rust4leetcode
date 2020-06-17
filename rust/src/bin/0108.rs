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

fn go(a: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if a.len() == 0 {
        return None;
    }
    let mid = a.len() / 2;
    let left = go(&a[..mid]);
    let right = go(&a[mid + 1..]);
    let mut node = TreeNode::new(a[mid]);
    node.left = left;
    node.right = right;
    return Some(Rc::new(RefCell::new(node)));
}

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        go(&nums)
    }
}

fn main() {
    println!("good");
    Solution::sorted_array_to_bst(vec!(1, 2, 3, 4, 5));
}