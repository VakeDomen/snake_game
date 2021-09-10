pub struct Body {
    head: Option<Node>
} 


pub struct Node {
    body: [u32; 2],
    tail: Option<Box<Node>>,
    eaten: bool,
}


impl Body {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    pub fn insert(&mut self, body: [u32; 2]) -> () {
        match &mut self.head {
            Some(head) => head.insert(body),
            None => self.head = Some(Node::new(body)),
        }
    }

    pub fn mark_eaten(&mut self, pos: [u32; 2]) -> () {
        match &mut self.head {
            Some(child) => child.mark_eaten(),
            None => self.insert(pos),
        }
    }

    pub fn move_body(&mut self, pos: [u32; 2]) -> () {
        match &mut self.head {
            Some(child) => child.move_node(pos),
            None => (),
        }
    }
}

impl Node {
    fn new(body: [u32; 2]) -> Self {
        Self {
            body: body,
            tail: None,
            eaten: false,
        }
    }

    fn insert(&mut self, body: [u32; 2]) -> () {
        match &mut self.tail {
            Some(child) => child.insert(body),
            None => self.tail = Some(Box::new(Node::new(body))),
        }
    }

    fn mark_eaten(&mut self) -> () {
        match &mut self.tail {
            Some(child) => child.mark_eaten(),
            None => self.eaten = true,
        }
    }

    fn move_node(&mut self, pos: [u32; 2]) -> () {
        match &mut self.tail {
            Some(child) => child.move_node(self.body),
            None => {
                if self.eaten {
                   self.eaten = false;
                   self.insert(self.body);
                }
            },
        }
        self.body = pos;
    }
}
