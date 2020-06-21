use std::ops::DerefMut;

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

fn vec2list(a: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(a[0])));
    let mut no = &mut head;
    for i in 1..a.len() {
        let now = Box::new(ListNode::new(a[i]));
        let last = no.as_deref_mut().unwrap();
        last.next = Some(now);
        no = &mut last.next;
    }
    return head;
}

fn print_link(link: &Option<Box<ListNode>>) {
    let mut i = link;
    loop {
        if i.is_none() {
            break;
        }
        let node = i.as_ref().unwrap();
        print!("{}->", node.val);
        i = &node.next;
    }
    println!()
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut lhs, mut rhs) = (l1, l2);
        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;

        while let (Some(lnode), Some(rnode)) = (lhs.as_ref(), rhs.as_ref()) {
            if lnode.val <= rnode.val {
                tail.next = lhs;
                tail = tail.next.as_mut().unwrap();
                lhs = tail.next.take();
            } else {
                tail.next = rhs;
                tail = tail.next.as_mut().unwrap();
                rhs = tail.next.take();
            }
        }

        tail.next = if lhs.is_some() { lhs } else { rhs };
        head.next
    }
}

fn main() {
    let x = vec2list(&vec!(1, 2, 3));
    let y = vec2list(&vec!(2, 4, 5));
    print_link(&x);
    print_link(&y);
    let z = Solution::merge_two_lists(x, y);
    print_link(&z);
}