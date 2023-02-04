mod linked_list;
use linked_list::LinkedList;

fn main() {
    let mut linked_list = LinkedList::<i32>::new();
    linked_list.append(4);
    linked_list.append(5);
    linked_list.append(6);
    println!("{}", linked_list.get_element(0).unwrap());
    println!("{}", linked_list.get_element(1).unwrap());
    println!("{}", linked_list.get_element(2).unwrap());

    linked_list.replace(0, 7).unwrap();
    println!("{}", linked_list.get_element(0).unwrap());
}
