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

}

impl<T: Debug> LinkNode<T> {
    pub fn print_list(&mut self){
        let mut current_node = self;

        loop {
            match &mut current_node.next {
                Some(next_node) => {
                    print!("{:?} -> ", current_node.val);
                    current_node = next_node;
                },
                None => {
                    println!("{:?}", current_node.val);
                    break;
                }
            }
        }
    }
}

impl<T: Clone> LinkNode<T> {
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

    pub fn remove_from_start(&mut self) {
        *self = *self.next.take().unwrap();
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