// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

struct Solution;

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

fn vec2list(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(v[0])));
    let mut h = &mut head;
    for i in 1..v.len() {
        let node = h.as_deref_mut().unwrap();
        node.next = Some(Box::new(ListNode::new(v[i])));
        h = &mut node.next;
    }
    return head;
}

fn list2string(li: &Option<Box<ListNode>>) -> String {
    let mut s: String = "".to_string();
    let mut i = li;
    loop {
        match i {
            None => break,
            Some(ii) => {
                s += &format!("{}", ii.val);
                i = &ii.next;
            }
        }
    }
    return s;
}

fn list2vec(li: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut i = li;
    let mut a = Vec::<i32>::new();
    loop {
        if i.is_none() {
            break;
        }
        let ii = i.as_deref().unwrap();
        let x = ii.val;
        a.push(x);
        i = &ii.next;
    }
    return a;
}

fn vec_add(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut c = Vec::<i32>::new();
    let mut enter = 0;
    let end = if a.len() > b.len() { a.len() } else { b.len() };
    for i in 0..end {
        let x = if i >= a.len() { 0 } else { a[i] };
        let y = if i >= b.len() { 0 } else { b[i] };
        let val = x + y + enter;
        enter = val / 10;
        c.push(val % 10);
    }
    if enter > 0 {
        c.push(enter);
    }
    return c;
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut a = list2vec(&l1);
        // a.reverse();
        let mut b = list2vec(&l2);
        // b.reverse();
        let mut c = vec_add(a, b);
        // c.reverse();
        return vec2list(c);
    }
}

fn main() {
    let l1 = vec2list(vec!(2, 4, 3));
    let l2 = vec2list(vec!(5, 6, 4));
    let ans = Solution::add_two_numbers(l1, l2);
    println!("{}", list2string(&ans));
}
