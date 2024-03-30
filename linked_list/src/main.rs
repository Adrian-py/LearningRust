mod linked_list;

use linked_list::LinkedList;

fn main() {
    let mut linked_list = LinkedList::<i32>::new();

    linked_list.push_end(3);
    linked_list.push_end(4);
    linked_list.push_start(12);
    linked_list.print_list();
    linked_list.pop_end();
    linked_list.print_list();
    linked_list.pop_start();
    linked_list.print_list();
}
