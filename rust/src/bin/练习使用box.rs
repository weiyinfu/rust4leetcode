use std::ops::{Deref, DerefMut};

fn main() {
    let mut x = Box::new(1);
    let mut y = x.to_owned();
    *y = 3;
    println!("{}", *x);
    let x = Some(Box::new(1));
    let mut y = x.to_owned();
    let z = y.as_deref_mut().unwrap();
    *z = 10;
    println!("{}", x.unwrap());
}