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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut sta = vec!();
        sta.push(&root);
        let mut ans = vec!();
        while !sta.is_empty() {
            let mut next_level = vec!();
            let mut cur_level = vec!();
            while !sta.is_empty() {
                let now = sta.remove(0);
                if now.is_none() {
                    continue;
                }
                unsafe {
                    let node = &*now.as_deref().unwrap().as_ptr();
                    cur_level.push(node.val);
                    next_level.push(&node.left);
                    next_level.push(&node.right);
                }
            }
            if cur_level.len() > 0 {
                ans.push(cur_level);
            }
            sta = next_level;
        }
        ans.reverse();
        ans
    }
}

fn main() {
    println!("good");
}