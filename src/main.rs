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
    println!("Here is that linked list with a replaced element: {}", linked_list);

    linked_list.insert(1, 3).unwrap();
    println!("Here is a linked list with an inserted element: {}", linked_list);
    linked_list.remove(1).unwrap();
    println!("Here is a linked list with a removed element: {}", linked_list);

    linked_list.clear();
    println!("Here is an empty linked list: {}", linked_list);
}
