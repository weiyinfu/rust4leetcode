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

fn go(a: &Vec<i32>, beg: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if beg > end {
        return vec!(None);
    }
    if beg == end {
        return vec!(Some(Rc::new(RefCell::new(TreeNode::new(a[beg as usize])))));
    }
    let mut ans = vec!();
    for i in beg..=end {
        let left_sons = go(a, beg, i - 1);
        let right_sons = go(a, i + 1, end);
        for left in left_sons.iter() {
            for right in right_sons.iter() {
                let mut root = Some(Rc::new(RefCell::new(TreeNode::new(a[i as usize]))));
                {
                    let mut node = root.as_deref().unwrap().borrow_mut();
                    node.left = left.clone();
                    node.right = right.clone();
                }
                ans.push(root);
            }
        }
    }
    ans
}

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut a = vec!();
        if n == 0 {
            return vec!();
        }
        for i in 1..=n {
            a.push(i);
        }
        go(&a, 0, (a.len() - 1) as i32)
    }
}

fn main() {
    let ans = Solution::generate_trees(3);
    for i in ans {
        println!("{:?}", i);
    }
}