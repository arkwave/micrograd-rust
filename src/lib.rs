use std::ops::{Add, Mul};

#[derive(Clone)]
pub struct Node {
    pub value: i32,
    pub children: Vec<Node>, // had initially set this to be HashSet, but they don't have a iter_mut() method. 
    pub grad: f32,
    pub backward: Backward,  
    pub op: String, 
}

#[derive(Clone)]
pub struct Backward {
    pub backward: fn(&mut Node) -> f64, 
}

impl Backward {
    pub fn new() -> Backward {
        Backward { backward: Backward::default}
    }

    pub fn default(_: &mut Node) -> f64 {
        0.0 
    }
}


// define a Pow trait for Nodes; allows for exponentiation
pub trait Pow<Node> {
    fn pow(self, rhs: &Node) -> Node;
}

impl Pow<Node> for &Node {
    fn pow(self, rhs: &Node) -> Node {
        Node::new(self.value.pow(rhs.value as u32)) 
    }
}


// overload the + and * operators for Nodes
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
        Node { value, children, grad: 0.0, op: String::from(""), backward: Backward::new() }
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
        self.backward = Backward { backward } 
    }
}
