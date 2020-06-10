fn main() {
    let x: Vec<char> = "abc".to_string().chars().collect();
    let y: Vec<char> = "abc".to_string().chars().collect();
    println!("{}", x == y)
}