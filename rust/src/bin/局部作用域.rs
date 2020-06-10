fn main() {
    let mut x=3;
    {
        let x=4;
    }
    println!("{}",x);
}