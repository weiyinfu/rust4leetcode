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

fn go<'long>(mut j: &mut Option<Box<ListNode>>, cur: &mut &mut ListNode) {
    //移动一格
    let node = j.as_deref_mut().unwrap();
    let mut temp = node.next.take();
    cur.next = j.take();
    *j = temp.take();
    //这句话怎么改才能改对呀
    // *cur = &mut cur.next.as_deref_mut().unwrap();
}


impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            return None;
        }
        if l1.is_none() && l2.is_some() {
            return l2;
        }
        if l1.is_some() && l2.is_none() {
            return l1;
        }
        let mut i = l1;
        let mut j = l2;
        let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(-1)));
        let mut cur = head.as_deref_mut().unwrap();
        while i.is_some() || j.is_some() {
            if i.is_none() {
                go(&mut j, &mut cur);
                continue;
            }
            if j.is_none() {
                go(&mut i, &mut cur);
                continue;
            }
            let ii = i.as_deref().unwrap();
            let jj = j.as_deref().unwrap();
            if ii.val < jj.val {
                go(&mut i, &mut cur);
            } else {
                go(&mut j, &mut cur);
            }
        }
        head.as_deref_mut().unwrap().next.take()
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