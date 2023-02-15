use micrograd_rust::Node;


fn main() {
    let n1 = Node::new(5);
    let n2 = Node::new(3);
    let n3 = &n1 + &n2;
    let n4 = &n1 * &n2;
    println!("n3: {:?}", n3);
    println!("n4: {:?}", n4);
    println!("n3's children: {:?}", n3.children);

}
