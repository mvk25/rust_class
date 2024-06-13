struct Node {
    value: i32,
    next: Option<Box<Node>>
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            next: None
        }
    }

    fn append(&mut self, value: i32) {
        match self.next {
            Some(ref mut next_node) => next_node.append(value),
            None => self.next = Some(Box::new(Node::new(value))),
        }
    }

    fn print(&self) {
        print!("{}", self.value);
        if let Some(ref next_node) = self.next {
            next_node.print();
        }
    }
}


fn main() {
    let mut head = Node::new(1);


    head.append(2);
    head.append(3);
    head.append(4);

    head.print();
}