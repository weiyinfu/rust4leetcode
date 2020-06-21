use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::BorrowMut;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn vec2tree(a: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let mut b: Vec<_> = a.iter().map(|x| {
        match *x {
            -1 => None,
            _ => Some(Rc::new(RefCell::new(TreeNode::new(*x)))),
        }
    }).collect();
    for i in 0..b.len() {
        if b[i].is_none() {
            continue;
        }
        let mut node = b[i].as_deref().unwrap().borrow_mut();
        if i * 2 + 1 < b.len() && b[i * 2 + 1].is_some() {
            node.left = Some(b[i * 2 + 1].as_ref().unwrap().clone());
        }
        if i * 2 + 2 < b.len() && b[i * 2 + 2].is_some() {
            node.right = Some(b[i * 2 + 2].as_ref().unwrap().clone());
        }
    }
    b[0].take()
}

fn print_tree(tr: &Option<Rc<RefCell<TreeNode>>>) {
    //以美观的方式打印一棵树
    fn go(prefix: i32, tr: &Option<Rc<RefCell<TreeNode>>>) {
        let p = ' '.to_string().repeat(prefix as usize);
        if tr.is_none() {
            println!("{}_", p);
            return;
        }
        let node = tr.as_deref().unwrap().borrow();
        println!("{}{}", p, node.val);
        go(prefix + 1, &node.left);
        go(prefix + 1, &node.right);
    }
    go(0, tr);
}

struct Solution;

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        //非递归中序遍历，找到两个反常值，第一个值突然由大变小，第二个值突然由小变大
        let now = root;
        let mut sta = vec!((now.as_ref().unwrap().clone(), false));
        let mut a = vec!();
        while !sta.is_empty() {
            let (now, visited) = sta.pop().unwrap();
            if visited {
                a.push(now);
                continue;
            }
            if now.borrow().right.is_some() {
                sta.push((now.borrow().right.as_ref().unwrap().clone(), false));
            }
            sta.push((now.clone(), true));

            if now.borrow().left.is_some() {
                sta.push((now.borrow().left.as_ref().unwrap().clone(), false));
            }

        }
        let mut bad = vec!();
        for i in 0..a.len() - 1 {
            println!("{}", a[i].borrow().val);
            let now = a[i].clone();
            let next = a[i + 1].clone();
            if now.borrow().val > next.borrow().val {
                bad.push(now);
                bad.push(next);
            }
        }
        // println!("bad length {}", bad.len());
        unsafe {
            let mut x = &mut *bad.first().unwrap().borrow_mut().as_ptr();
            let mut y = &mut *bad.last().unwrap().borrow_mut().as_ptr();
            let temp = x.val;
            x.val = y.val;
            y.val = temp;
        }
    }
}

fn main() {
    let mut ans = vec2tree(&[1, 3, -1, -1, 2]);
    print_tree(&ans);
    Solution::recover_tree(&mut ans);
}

