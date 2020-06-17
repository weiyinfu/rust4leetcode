// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}
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


fn solve(a: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if a.len() == 0 {
        return None;
    }
    let mid = a.len() / 2;
    let left = solve(&a[..mid]);
    let right = solve(&a[mid + 1..]);
    let mut node = TreeNode::new(a[mid]);
    node.left = left;
    node.right = right;
    return Some(Rc::new(RefCell::new(node)));
}

fn go(head: &Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut sz = 0;
    let mut i = head;
    let mut a = vec!();
    while i.is_some() {
        a.push(i.as_deref().unwrap().val);
        i = &i.as_deref().unwrap().next;
        sz += 1;
    }
    solve(&a)
}

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        go(&head)
    }
}

fn main() {
    println!("good");
}