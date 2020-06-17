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

fn go(root: &Option<Rc<RefCell<TreeNode>>>, sum: i32, ans: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
    if root.is_none() {
        return;
    }
    let node = root.as_deref().unwrap().borrow();
    if node.left.is_none() && node.right.is_none() {
        if sum == node.val {
            path.push(node.val);
            ans.push(path.clone());
            path.pop();
        } else {
            return;
        }
    }
    if node.left.is_some() {
        path.push(node.val);
        go(&node.left, sum - node.val, ans, path);
        path.pop();
    }
    if node.right.is_some() {
        path.push(node.val);
        go(&node.right, sum - node.val, ans, path);
        path.pop();
    }
}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec!();
        }
        let mut ans = vec!();
        let mut path = vec!();
        go(&root, sum, &mut ans, &mut path);
        ans
    }
}


fn main() {
    println!("haha");
}