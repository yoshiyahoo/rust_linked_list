use crate::list::*;

//a struct with <T> works similarly to a template in C++
pub struct Node<T> {
    //the value at this current node.
    val: T,

    //A box is a smart pointer to the thing inside the box, a node with <T>
    //Sometimes, we might not be pointing to a node, but pointing to nothing
    //Wrapping an option around the pointer allows us to point to nothing
    next: Option<Box<Node<T>>>
}
pub struct LinkedList<T> {
    //the length of the LinkedList
    len: u32,

    //the beginning of the LinkedList. More info about the syntax above.
    head: Option<Box<Node<T>>>
}
