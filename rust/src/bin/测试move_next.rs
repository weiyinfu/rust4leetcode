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

fn move_next(i: &mut &Option<Box<ListNode>>) {
    *i = &i.as_ref().unwrap().next;
}

fn length(link: &mut Option<Box<ListNode>>) -> i32 {
    let mut sz = 0;
    let mut i = &mut &*link;
    loop {
        if i.is_none() {
            break;
        }
        move_next(&mut i);
        sz += 1;
    }
    sz
}

fn main() {
    let mut link = vec2list(&vec!(1, 2, 3, 4));
    print_link(&link);
    let sz = length(&mut link);
    println!("{}", sz);
}