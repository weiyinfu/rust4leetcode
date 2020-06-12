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

fn vec2list(a: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(a[0])));
    let mut i = &mut head;
    for j in 1..a.len() {
        let node = i.as_deref_mut().unwrap();
        let new_node = Some(Box::new(ListNode::new(a[j])));
        node.next = new_node;
        i = &mut node.next;
    }
    head
}

fn print_link(a: &Option<Box<ListNode>>) {
    let mut i = a;
    while i.is_some() {
        let node = i.as_deref().unwrap();
        print!("{}->", node.val);
        i = &node.next;
    }
}

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut i = &mut head;
        while i.is_some() {
            let now = i.as_deref_mut().unwrap();
            if now.next.is_none() {
                break;
            }
            let next = now.next.as_deref_mut().unwrap();
            if now.val == next.val {
                now.next = next.next.take();
            } else {
                i = &mut i.as_deref_mut().unwrap().next;
            }
        }
        head
    }
}

fn main() {
    let link = vec2list(vec![1, 2, 2, 3, 4]);
    let link = Solution::delete_duplicates(link);
    print_link(&link);
}