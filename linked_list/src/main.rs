mod linked_list;

use linked_list::LinkNode;

fn main() {
    let mut head = LinkNode::<u32>::new(1);
    head.add_to_start(2); 
    head.add_to_start(3); 
    head.add_to_end(4); 
    head.add_to_end(5);
    head.print_list();
    head.remove_from_start();
    head.print_list();
    head.remove_from_end();
    head.print_list();
}
