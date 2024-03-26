mod link_node;

use std::fmt::Debug;

use link_node::LinkNode;

pub struct LinkedList<T> {
    pub head: Option<LinkNode<T>>,
    pub len: u32,
}

impl<T: Clone + Debug> LinkedList<T> {
    pub fn new(val: T) -> Self {
        Self {
            head: Some(LinkNode::new(val)),
            len: 1,
        }
    }

    pub fn print_list(&mut self) {
        match &mut self.head {
            Some(head) => {
                let mut current_node = head;
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
            },
            None => {
                println!("Linked List is empty!");
            }
        }
    }

    pub fn add_to_start(&mut self, val: T) {
        let mut new_node = LinkNode::new(val);

        match &mut self.head {
            Some(current_head) => {
                new_node.next = Some(Box::new(current_head.clone()));
                self.head = Some(new_node);
            },
            None => {
                self.head = Some(new_node);
            },
        }
    }

    pub fn add_to_end(&mut self, val: T) {
        match &mut self.head {
            Some(first_node) => {
                first_node.add_to_end(val);
            },
            None => {
                let new_node = LinkNode::new(val);
                self.head = Some(new_node);
            }
        }
    }

    pub fn remove_from_start(&mut self) -> bool {
        match &mut self.head {
            Some(head) => {
                match &mut head.next {
                    Some(next_head) => {
                        self.head = Some(*next_head.clone());
                        true
                    },
                    None => {
                        self.head = None;
                        true
                    }
                }
            },
            None => {
                println!("Linked List is already empty!");
                false
            }
        }
    }

    pub fn remove_from_end(&mut self) -> bool {
        match &mut self.head {
            Some(head_node) => {
                head_node.remove_from_end();
                true
            },
            None => {
                println!("Linked list is already empty!");
                false
            }
        } 
    }
}
