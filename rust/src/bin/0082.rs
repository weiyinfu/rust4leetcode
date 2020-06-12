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
        let mut mock = Some(Box::new(ListNode::new(0)));
        mock.as_deref_mut().unwrap().next = head;
        let mut i = &mut mock;
        while i.is_some() {
            let now = i.as_deref_mut().unwrap();
            if now.next.is_none() {
                break;
            }
            let val = now.next.as_deref().unwrap().val;
            let mut j = &mut now.next;
            let mut cnt = 0;
            while j.is_some() && j.as_deref().unwrap().val == val {
                j = &mut j.as_deref_mut().unwrap().next;
                cnt += 1;
            }
            // println!("val={} cnt={}", val, cnt);
            if cnt > 1 {
                //如果是独一无二的
                now.next = j.take();
            } else {
                i = &mut i.as_deref_mut().unwrap().next;
            }
        }
        mock.as_deref_mut().unwrap().next.take()
    }
}

fn main() {
    let link = vec2list(vec![1,  2, 2, 3, 4]);
    let link = Solution::delete_duplicates(link);
    print_link(&link);
}