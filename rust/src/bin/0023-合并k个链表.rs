use std::collections::BinaryHeap;
use std::cmp::Ordering;

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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        impl Ord for ListNode {
            fn cmp(&self, other: &Self) -> Ordering {
                if self.val == other.val {
                    Ordering::Equal
                } else if self.val > other.val {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }
        impl PartialOrd for ListNode {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        let mut q = BinaryHeap::<Box<ListNode>>::new();
        for i in lists {
            if i.is_none() {
                continue;
            }
            let it = i.unwrap();
            q.push(it);
        }
        let mut head = Some(Box::new(ListNode::new(-1)));
        let mut cur = &mut head;
        while !q.is_empty() {
            let mut i = q.pop().unwrap();
            println!("{}", i.val);
            let next = i.next.take();
            if next.is_some() {
                q.push(next.unwrap());
            }
            let now = cur.as_deref_mut().unwrap();
            now.next = Some(i);
            cur = &mut now.next;
        }
        head.unwrap().next.take()
    }
}

fn main() {
    let link1 = vec2list(&vec!(1, 4, 5));
    let link2 = vec2list(&vec!(1, 3, 4));
    let link3 = vec2list(&vec!(4, 6));
    let a = vec!(link1, link2, link3);
    let ans = Solution::merge_k_lists(a);
    print_link(&ans);
}