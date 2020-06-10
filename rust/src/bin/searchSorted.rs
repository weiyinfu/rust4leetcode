fn main() {
    let a = vec!(1, 2, 3, 3, 7);
    let b = a.binary_search(&5);
    let c = a.binary_search(&10);
    let d = a[3..].binary_search(&7);
    println!("{}", d.unwrap());
    let k=d.unwrap();
    //如果没有找到会返回error
}