use std::collections::HashMap;

fn test1() {
    let mut ma = HashMap::new();
    ma.insert(&"one", 1);
    println!("{}", ma.get(&"one").unwrap());
}

fn test2() {
    let mut ma = HashMap::new();
    for i in 0..5 {
        //报错：x活得不够长
        let x = "haha".to_string();
        ma.insert(&x, 0);
    }
    println!("{:?}", ma);
}

fn main() {
    test2();
}