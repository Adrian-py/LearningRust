use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct LinkNode<T> {
    pub val: T,
    pub next: Option<Box<LinkNode<T>>>,
}

impl<T> LinkNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val: val,
            next: None,
        }
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

    pub fn remove_from_end(&mut self) -> bool {
        match &mut self.next {
            Some(next_node) => {
                if next_node.remove_from_end() == true {
                    self.next = None;
                }
                false
            },
            None => {
                true
            },
        }
    }
}
