fn main() {
    let mut x = Some(Box::new(3));
    let z = &mut x;
    while z.is_some() {
        let y = z.as_deref_mut().unwrap();
        *y += 1;
        println!("haha");
    }
}