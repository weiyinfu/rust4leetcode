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
    let mut no = head.as_mut();
    for i in 1..a.len() {
        let now = Box::new(ListNode::new(a[i]));
        let last = no.unwrap();
        last.next = Some(now);
        no = last.next.as_mut();
    }
    return head;
}

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut size = 0;
        let mut now = &head;
        while now.is_some() {
            let node = now.as_deref().unwrap();
            now = &node.next;
            size += 1;
        }
        if size == n {
            return head.unwrap().next;
        }
        // println!("link size ={}", size);
        size = size - n;
        // println!("removing {}", size);
        let mut now = &mut head;
        while size > 1 {
            let node = now.as_deref_mut().unwrap();
            now = &mut node.next;
            size -= 1;
        }
        let mut node = now.as_deref_mut().unwrap();
        let nex = node.next.to_owned().unwrap();
        node.next = nex.next.to_owned();
        // println!("removed {} {}", nex.val, node.val);
        head
    }
}

fn main() {
    let li = vec2list(&vec!(1, 2, 3, 4, 5));
    let a = Solution::remove_nth_from_end(li, 3);
    println!("{:?}", a);
}