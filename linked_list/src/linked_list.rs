use std::fmt::Debug;

#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Option<Box<LinkNode<T>>>,
}

#[derive(Debug)]
pub struct LinkNode<T> {
    pub val: T,
    pub next: Option<Box<LinkNode<T>>>,
}

impl<T: Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
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
                        }
                        None => {
                            println!("{:?}", current_node.val);
                            break;
                        }
                    }
                }
            }
            None => {
                println!("Linked List is empty!");
            }
        }
    }

    pub fn push_start(&mut self, val: T) {
        let new_node = LinkNode {
            val: val,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop_start(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                self.head = head.next;
                Some(head.val)
            }
            None => None,
        }
    }

    pub fn push_end(&mut self, val: T) {
        let new_node = Some(Box::new(LinkNode {
            val: val,
            next: None,
        }));
        match self.get_tail() {
            Some(last_node) => {
                last_node.next = new_node;
            }
            None => {
                self.head = new_node;
            }
        }
    }

    pub fn pop_end(&mut self) -> Option<T> {
        // check if there is only one node in the list
        if let Some(head) = &self.head {
            if head.next.is_none() {
                return Some(self.head.take().unwrap().val);
            }
        }

        // retrieve second to last node and change .next
        let mut current_node = &mut self.head;

        while let Some(node) = current_node {
            match node.next.as_mut() {
                Some(next_node) => {
                    if next_node.next.is_none() {
                        return Some(node.next.take().unwrap().val);
                    }
                }
                None => return None,
            }
            current_node = &mut node.next;
        }
        None
    }

    pub fn get_tail(&mut self) -> Option<&mut Box<LinkNode<T>>> {
        let mut current_node = &mut self.head;
        while let Some(node) = current_node {
            if node.next.is_none() {
                return Some(node);
            }
            current_node = &mut node.next;
        }
        None
    }
}
