use std::ops::{Add, Mul}; 
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Node {
    pub value: i32,
    pub children: HashSet<Node>,
    grad: f32,
    pub op: String, 

}

impl Add for &Node {
    type Output = Node;
    fn add(self, other: &Node) -> Node {
        Node::new(self.value + other.value)
    }
}

impl Mul for &Node {
    type Output = Node;
    fn mul(self, rhs: Self) -> Self::Output {
        Node::new(self.value * rhs.value) 
    }
}

impl Node {
    pub fn new(value: i32) -> Node  {
        let children: HashSet<Node> = HashSet::new();
        Node { value, children, grad: 0.0, op: String::from("")}
    }

    pub fn backward(&mut self) {
        if self.grad == 0.0 {
            self.grad = 1.0 
        } 
        for child in self.children.iter() {
            child.backward();
        }
    }
    pub fn topological_sort(&self) -> Vec<Node> {
    
        return vec![] 
    }
}
