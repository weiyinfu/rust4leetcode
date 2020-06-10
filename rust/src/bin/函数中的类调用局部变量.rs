fn main() {
    let x = "haha";
    struct Node;
    impl Node {
        fn haha() {
            //会报错
            // println!("{}", x);
        }
    }
    Node::haha();
}