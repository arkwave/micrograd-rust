use micrograd_rust::Node;
use micrograd_rust::Pow;


fn main() {
    let n1 = Node::new(5);
    let n2 = Node::new(2);
    let n3 = &n1 * &n2; 
    let mut n4 = n3.pow(&Node::new(2));
    n4.backward(); 
    println!("n1.grad: {}", n1.grad);
}
