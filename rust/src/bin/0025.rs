// Definition for singly-linked list.
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

fn vec2list(v: &Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(v[0])));
    let mut cur = &mut head;
    let mut i = 1;
    while i < v.len() {
        let now = Some(Box::new(ListNode::new(v[i])));
        let node = cur.as_deref_mut().unwrap();
        node.next = now;
        cur = &mut node.next;
        i += 1;
    }
    head
}

fn print_link(link: &Option<Box<ListNode>>) {
    let mut i = link;
    loop {
        if i.is_none() {
            break;
        }
        print!("{}->", i.as_deref().unwrap().val);
        i = &i.as_deref().unwrap().next;
    }
    println!()
}

struct Solution;

fn reverse(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //reverse整个链表
    let mut fake = Some(Box::new(ListNode::new(-1)));//来一个哨兵单元
    let mut h = head.take();
    while h.is_some() {
        let mut header = h.as_deref_mut().unwrap().next.take();
        let now_node = fake.as_deref_mut().unwrap();
        let mut temp = now_node.next.take();
        now_node.next = h.take();
        let first = now_node.next.as_deref_mut().unwrap();
        first.next = temp.take();
        if header.is_none() {
            //记录最后一个结点
            break;
        } else {
            h = header.take();
        }
    }
    fake.unwrap().next.take()
}

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 1 {
            return head;
        }
        let mut fake = Some(Box::new(ListNode::new(-1)));
        fake.as_mut().unwrap().next = head;
        let mut cur = &mut fake;
        while cur.is_some() {
            //寻找第k个元素
            let mut next = &mut *cur;
            for i in 0..k {
                next = &mut next.as_mut().unwrap().next;
                if next.is_none() {
                    break;
                }
            }
            if next.is_none() {
                //如果是空，那么直接添加
                break;
            }
            //一个take就已经把链表断开了
            let nex = next.as_deref_mut().unwrap().next.take();//接管后续
            let sub_link = reverse(&mut cur.as_mut().unwrap().next.take());
            cur.as_deref_mut().unwrap().next = sub_link;
            //因为生命周期问题，此处应该让cur自己往后跑
            for i in 0..k {
                cur = &mut cur.as_mut().unwrap().next;
            }
            cur.as_deref_mut().unwrap().next = nex;
        }
        fake.unwrap().next.take()
    }
}


fn main() {
    let mut link = vec2list(&vec!(1, 2, 3, 4, 5));
    let ans = Solution::reverse_k_group(link, 2);
    // let ans = reverse(&mut link);
    print_link(&ans);
    // print_link(&ans);
}