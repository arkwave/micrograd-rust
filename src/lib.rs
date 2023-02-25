use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
pub struct Node {
    pub value: i32,
    pub children: Vec<Node>, // had initially set this to be HashSet, but they don't have a iter_mut() method. 
    pub grad: f32,
    pub backward: fn(&mut Self) -> f64,  
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
        let children: Vec<Node> = Vec::new();
        Node { value, children, grad: 0.0, op: String::from(""), backward: fn(&mut Self) -> f64 { 0.0 } }
    }

    pub fn backward(&mut self) {
        if self.grad == 0.0 {
            self.grad = 1.0 
        }
        for child in self.children.iter_mut() {
            child.backward();
        }
        self.children = Vec::new(); 
    }
    pub fn topological_sort(&self) -> Vec<Node> {
    
        return vec![] 
    }

    pub fn set_backward(&mut self, backward: fn(&mut Self) -> f64) {
        self.backward = backward 
    }
}
