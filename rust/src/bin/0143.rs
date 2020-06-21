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

struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }
        let mut a = vec!();
        let mut i = head.take();
        while i.is_some() {
            let mut next = i.as_deref_mut().unwrap().next.take();
            a.push(i.take());
            i = next.take();
        }
        let mut i = 0;
        let mut j = a.len() - 1;
        let mut b = vec!();
        while i < j {
            b.push(i);
            b.push(j);
            j -= 1;
            i += 1;
        }
        if i == j {
            b.push(i);
        }

        for i in (0..b.len() - 1).rev() {
            a[b[i]].as_deref_mut().unwrap().next = a[b[i + 1]].take();
        }
        *head = a[0].take()
    }
}

fn vec2list(a: Vec<i32>) -> Option<Box<ListNode>> {
    let mut node = Some(Box::new(ListNode::new(a[0])));
    let mut i = &mut node;
    for j in 1..a.len() {
        let now = i.as_deref_mut().unwrap();
        now.next = Some(Box::new(ListNode::new(a[j])));
        i = &mut now.next;
    }
    node
}

fn main() {
    println!("good");
    let mut li = vec2list(vec!(1, 2, 3, 4, 5));
    Solution::reorder_list(&mut li);
    println!("{:?}", li);
}