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

fn vec2list(a: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(a[0])));
    let mut cur = &mut head;
    for i in 1..a.len() {
        let node = Some(Box::new(ListNode::new(a[i])));
        cur.as_deref_mut().unwrap().next = node;
        cur = &mut cur.as_deref_mut().unwrap().next;
    }
    head
}

fn print_link(a: Option<Box<ListNode>>) {
    let mut i = &a;
    while i.is_some() {
        let node = i.as_deref().unwrap();
        let nex = &node.next;
        print!("{}->", node.val);
        i = nex;
    }
    println!()
}

struct Solution;

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }
        let mut i = 0;
        let mut no = &head;
        while no.is_some() {
            i += 1;
            no = &no.as_deref().unwrap().next;
        }
        let sz = i;
        k %= sz;
        k = (sz - k) % sz;
        if sz == 1 || k == 0 {
            return head;
        }
        let mut no = &mut head;
        let mut i = 1;
        while i < k {
            i += 1;
            no = &mut no.as_deref_mut().unwrap().next;
        }
        let mut ans = no.as_deref_mut().unwrap().next.take();
        let mut no = &mut ans;
        loop {
            let node = no.as_deref_mut().unwrap();
            let mut nex = &mut node.next;
            if nex.is_none() {
                //如果没有新元素了
                break;
            }
            no = &mut no.as_deref_mut().unwrap().next;
        }
        let mut tail = no;
        tail.as_deref_mut().unwrap().next = head.take();
        ans
    }
}

fn main() {
    let ans = Solution::rotate_right(vec2list(vec![1, 2, ]), 2);
    print_link(ans);
}