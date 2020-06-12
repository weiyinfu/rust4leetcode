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
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() || m == n {
            return head;
        }
        let mut counter = 1;
        let mut mock = Some(Box::new(ListNode::new(-1)));
        mock.as_deref_mut().unwrap().next = head;
        let mut i = &mut mock;
        while counter < m {
            i = &mut i.as_deref_mut().unwrap().next;
            counter += 1;
        }
        let mut pre = i;
        let mut i = pre.as_deref_mut().unwrap().next.take();
        while counter <= n {
            if i.is_none() {
                break;
            }
            let temp = pre.as_deref_mut().unwrap().next.take();
            let mut now = i.take();
            i = now.as_deref_mut().unwrap().next.take();
            now.as_deref_mut().unwrap().next = temp;
            pre.as_deref_mut().unwrap().next = now.take();
            counter += 1;
        }
        while pre.as_deref().unwrap().next.is_some() {
            pre = &mut pre.as_deref_mut().unwrap().next;
        }
        pre.as_deref_mut().unwrap().next = i;
        mock.as_deref_mut().unwrap().next.take()
    }
}

fn vec2list(a: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(a[0])));
    let mut i = &mut head;
    for j in 1..a.len() {
        let now = Some(Box::new(ListNode::new(a[j])));
        i.as_deref_mut().unwrap().next = now;
        i = &mut i.as_deref_mut().unwrap().next;
    }
    head
}

fn print_link(a: &Option<Box<ListNode>>) {
    let mut i = a;
    while i.is_some() {
        let now = i.as_deref().unwrap();
        print!("{}=>", now.val);
        i = &now.next;
    }
    println!();
}

fn main() {
    let link = vec2list(&vec!(1, 2, 3,));
    print_link(&link);
    let ll = Solution::reverse_between(link, 2, 4);
    print_link(&ll);
}