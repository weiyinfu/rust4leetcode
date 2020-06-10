use std::fmt::{Display, Formatter};
use std::fmt;

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

fn vec2list(v: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(v[0])));
    let mut cur = &mut head;
    let mut i = 1;
    while i < v.len() {
        let now = Some(Box::new(ListNode::new(v[i])));
        let node = cur.as_deref_mut().unwrap();
        node.next = now;
        cur = &mut node.next;
        i += 1;
    }
    head
}

fn print_link(link: &Option<Box<ListNode>>) {
    let mut i = link;
    loop {
        if i.is_none() {
            break;
        }
        print!("{}->", i.as_deref().unwrap().val);
        i = &i.as_deref().unwrap().next;
    }
    println!()
}

struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut a = Some(Box::new(ListNode::new(-1)));
        a.as_deref_mut().unwrap().next = head;
        let mut i = &mut a;
        loop {
            if i.is_none() {
                break;
            }
            let inode = i.as_deref_mut().unwrap();
            let mut x = inode.next.take();
            if x.is_none() {
                break;
            }
            let xnode = x.as_deref_mut().unwrap();
            let mut y = xnode.next.take();
            if y.is_none() {
                //在跳出之前需要把xnode添加到i上
                inode.next = x.take();
                break;
            }
            let ynode = y.as_deref_mut().unwrap();
            let mut z = ynode.next.take();
            xnode.next = z.take();
            ynode.next = x.take();
            inode.next = y.take();
            //此处难道就这么丑陋吗，x，y，z都被take无法使用了，只能从i向后走
            i = &mut inode.next.as_deref_mut().unwrap().next;
        }
        let node = a.as_deref_mut().unwrap();
        node.next.take()
    }
}

fn main() {
    let link = vec2list(&vec!(1, 2, 3, 4));
    print_link(&link);
    let ans = Solution::swap_pairs(link);
    print_link(&ans);
}