fn main() {
    let mut head = Some(Box::new(3));
    let mut p = &mut head;
    // let x = p.unwrap();
    //unwrap的参数是self，一定会发生move
}