use std::ptr::NonNull;

use crate::list::*;
//a struct with <T> works similarly to a template in C++
pub struct Node<T> {
    //the value at this current node.
    val: T,

    //NonNull is a smart pointer that's just a raw pointer without a null case
    //Sometimes, we might not be pointing to a node, but pointing to nothing
    //Wrapping an option around the pointer allows us to point to nothing
    next: Option<NonNull<Node<T>>>
}

#[derive(Default)]
pub struct LinkedList<T> {
    //the length of the LinkedList
    len: usize,

    //the beginning of the LinkedList. More info about the syntax above.
    head: Option<NonNull<Node<T>>>
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            val: elem,
            next: None
        }
    }
}

//this is where we implement the List trait functions
impl<T> List for LinkedList<T> {
    type Val = T;
    
    fn append(&mut self, elem: Self::Val) {
        unsafe {
            let mut curr = self.head;
            while curr != None {
                curr = (*curr.unwrap().as_ptr()).next;
            }
            let next_node = &mut Node::new(elem) as *mut Node<T>;  
            curr = NonNull::new(next_node);
            self.len += 1;
        }
    }

    fn clear(&self) {
        todo!();
    }

    fn get_element(&self, position: usize) -> Result<Self::Val, String> {
        let mut curr = self.head;
        
        if position > self.len || position < 0{
            Err(String::from("access error: index out of range"))
        }
        else {
            unsafe {
                for i in 0..position {
                    curr = (*curr.unwrap().as_ptr()).next;
                }
                Ok((*curr.unwrap().as_ptr()).val)
            }
        }

    }

    fn get_length(&self) -> usize {
        //we don't need a semicolon on every line, Rust can interpret this as a u32
        self.len
    }

    fn insert(&mut self, position: usize, elem: Self::Val) -> Result<(), String> {
        todo!();
    }

    fn is_empty(&self) -> bool {
        //we don't need a semicolon on every line, Rust can interpret this as a bool
        self.len == 0
    }

    fn remove(&mut self, position: usize) -> Result<(), String> {
        todo!();
    }

    fn replace(&mut self, position: usize, elem: Self::Val) -> Result<(), String> {
        todo!();
    }
}