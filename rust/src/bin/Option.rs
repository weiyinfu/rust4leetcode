
fn main() {
    let x = Some(Box::new(3));
    let y = x;
    println!("{}", x);
    let z=y;
}