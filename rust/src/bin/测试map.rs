use std::collections::HashMap;

fn test1() {
    let mut ma = HashMap::new();
    ma.insert(&"one", 1);
    println!("{}", ma.get(&"one").unwrap());
}

fn test3() {
    //会报错
    let mut ma: HashMap<&str, i32> = HashMap::new();
    let res = ma.get_mut("one");
    let mut temp = 0;
    let x = res.unwrap_or(&mut temp);
    *x = 5;
    println!("{}", ma.get("one").unwrap());
}

fn test2() {
    let mut ma = HashMap::new();
    ma.insert("haha".to_string(), 3);
    for i in 0..5 {
        //报错：x活得不够长
        let x = "haha".to_string();
        // ma.insert(&x, 0);
    }
    println!("{:?}", ma);
}

fn main() {
    test3();
}