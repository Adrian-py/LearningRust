mod linked_list;

use linked_list::LinkedList;

fn main() {
    let mut linked_list = LinkedList::<u32>::new(1);

    /*
        Should end up with:
        3 -> 2 -> 1 -> 5 -> 6
     */
    linked_list.add_to_start(2);
    linked_list.add_to_start(3);
    linked_list.add_to_start(4);
    linked_list.add_to_end(5);
    linked_list.add_to_end(6);
    linked_list.add_to_end(7);
    linked_list.print_list();
    linked_list.remove_from_start();
    linked_list.print_list();
    linked_list.remove_from_end();
    linked_list.print_list();
}
