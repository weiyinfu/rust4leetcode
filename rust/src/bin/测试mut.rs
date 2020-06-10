fn haha(mut x: i32) {
    x = 4;
}

fn main() {
    let x = 3;
    haha(x);
    println!("{}", x);
}