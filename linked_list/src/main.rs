#[derive(Clone, Debug, PartialEq)]
pub struct LinkNode<T> {
    pub val: T,
    pub next: Option<Box<LinkNode<T>>>,
}

impl<T: Clone> LinkNode<T> {
    fn new(val: T) -> Self {
        Self {
            val: val,
            next: None,
        }
    }

    pub fn add_to_start(&mut self, val: T) {
        let new_node = Self {
            val: val,
            next: Some(Box::new(self.clone())),
        };

        *self = new_node; 
    }

    pub fn add_to_end(&mut self, val: T) {
        match &mut self.next {
            Some(next_node) => {
                next_node.add_to_end(val);
            },
            None => {
                let new_node = Some(Box::new(Self {
                    val: val,
                    next: None,
                }));

                self.next = new_node;
            },
        }
    }
}

fn main() {
    let mut head: LinkNode<u32> = LinkNode::new(1);
    LinkNode::add_to_start(&mut head, 2); 
    LinkNode::add_to_start(&mut head, 3); 
    LinkNode::add_to_start(&mut head, 4); 
    LinkNode::add_to_end(&mut head, 5);
    LinkNode::add_to_end(&mut head, 6);
    println!("{:?}", head);
}
