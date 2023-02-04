mod linked_list;
use linked_list::LinkedList;

fn main() {
    let mut linked_list = LinkedList::<i32>::new();
    linked_list.append(4);
    println!("Here is a linked list: {}", linked_list);
    linked_list.append(5);
    println!("Here is a bigger one: {}", linked_list);
    linked_list.append(6);
    println!("And a bigger one: {}", linked_list);

    linked_list.replace(0, 7).unwrap();
    println!("Here is an element of a linked list: {}", linked_list.get_element(0).unwrap());
}
