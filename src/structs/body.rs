use piston_window::*;

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

    pub fn draw<G: Graphics>(&self, context: piston_window::Context, graphics: &mut G, w: u32, h: u32) -> () {
        match &self.head {
            Some(child) => child.draw(context, graphics, w, h),
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

    fn draw<G: Graphics>(&self, context: piston_window::Context, graphics: &mut G, w: u32, h: u32) -> () {
        match &self.tail {
            Some(child) => child.draw(context, graphics, w, h),
            None => (),
        }
        let part: [f64; 4] = [
            self.body[0] as f64 * w as f64,
            self.body[1] as f64 * h as f64,
            w as f64, 
            h as f64,
        ];
        rectangle(
            [0.0, 0.0, 1.0, 1.0], // red
            part,
            context.transform,
            graphics
        );
    }
}
