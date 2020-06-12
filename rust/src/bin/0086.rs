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
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut big = Some(Box::new(ListNode::new(0)));
        let mut small = Some(Box::new(ListNode::new(0)));
        let mut big_i = &mut big;
        let mut small_i = &mut small;
        let mut i = &mut head;
        while i.is_some() {
            println!("{}", i.as_deref().unwrap().val);
            if i.as_deref().unwrap().val < x {
                small_i.as_deref_mut().unwrap().next = i.take();
                small_i = &mut small_i.as_deref_mut().unwrap().next;
                i = &mut small_i;
            } else {
                big_i.as_deref_mut().unwrap().next = i.take();
                big_i = &mut big_i.as_deref_mut().unwrap().next;
                i = &mut big_i;
            }
            i = &mut i.as_deref_mut().unwrap().next;
        }
        small_i.as_deref_mut().unwrap().next = big.as_deref_mut().unwrap().next.take();
        small.as_deref_mut().unwrap().next.take()
    }
}

fn vec2list(a: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(a[0])));
    let mut j = &mut head;
    for i in 1..a.len() {
        let mut node = Some(Box::new(ListNode::new(a[i])));
        j.as_deref_mut().unwrap().next = node;
        j = &mut j.as_deref_mut().unwrap().next;
    }
    head
}

fn print_list(link: Option<Box<ListNode>>) {
    let mut i = &link;
    while i.is_some() {
        print!("{}=>", i.as_deref().unwrap().val);
        i = &i.as_deref().unwrap().next;
    }
    println!()
}

fn main() {
    println!("haha");
    let ans = Solution::partition(vec2list(vec!(1, 2, 3, 2, 3, 1, 4, 5)), 3);
    print_list(ans);
}