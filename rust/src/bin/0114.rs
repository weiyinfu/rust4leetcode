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

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut temp;
        {
            let mut node = root.as_deref().unwrap().borrow_mut();
            Solution::flatten(&mut node.left);
            Solution::flatten(&mut node.right);
            temp = node.right.take();
            node.right = node.left.take();
        }
        let mut i = &*root;
        loop {
            unsafe {
                let mut node = &mut *(i.as_deref().unwrap().as_ptr());
                if node.right.is_none() {
                    break;
                }
                i = &mut node.right;
            }
        }
        let mut it = i.as_deref().unwrap().borrow_mut();
        it.right = temp.take();
    }
}


fn main() {
    println!("hellow");
}