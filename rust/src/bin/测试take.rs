fn main() {
    let mut head = Some(Box::new(3));
    let mut p = &mut head;
    let x = p.take();
    println!("{}", head.is_none());
}