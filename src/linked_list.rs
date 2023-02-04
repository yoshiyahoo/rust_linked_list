use core::fmt;
use std::ptr::{null, null_mut};
use std::alloc::{alloc, dealloc, Layout};

//a struct with <T> works similarly to a template class in C++
pub struct Node<T> {
    //the value at this current node.
    val: T,

    //Sometimes, we might not be pointing to a node, but pointing to nothing
    //Wrapping an option around the pointer allows us to point to nothing
    next: *mut Node<T>
}


pub struct LinkedList<T> {
    //the length of the LinkedList
    len: usize,

    //the beginning of the LinkedList. More info about the syntax above.
    head: *mut Node<T>
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            val: elem,
            next: null_mut() as *mut Node<T>
        }
    }
}

impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        unsafe {
            let mut curr = self.head;
            loop {
                if (*curr).next == null_mut() {
                    buf += format!("{}", (*curr).val).as_str();
                    break;
                }
                else {
                    buf += format!("{} -> ", (*curr).val).as_str();
                    curr = (*curr).next;
                }
            }
        }
        write!(f, "{}", buf)
    }
}


//this is where we implement the List trait functions
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            head: null_mut() as *mut Node<T>,
        }
    }
    
    pub fn append(&mut self, elem: T) {
        let mut ptrs = vec![];
        unsafe {
            let mut curr = self.head;
            loop {
                if curr != null_mut() {
                    ptrs.push(curr);
                    curr = (*curr).next;
                }
                else {
                    let layout = Layout::new::<Node<T>>();
                    let ptr = alloc(layout);
                    *(ptr as *mut Node<T>) = Node::new(elem);
                    curr = ptr as *mut Node<T>;
                    ptrs.push(curr);

                    self.len += 1;
                    break;
                }
            }
            for i in (1..ptrs.len()).rev() {
                (*ptrs[i - 1]).next = ptrs[i];
            }
            self.head = ptrs[0];
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.head = null_mut();
    }

    pub fn get_element(&self, position: usize) -> Result<&T, &str> {
        if position >= self.len {
            return Err("Position out of range");
        }
        unsafe {
            let mut curr = self.head;
            for _i in 0..position {
                if curr != null_mut() {
                    curr = (*curr).next;
                }
                else {
                    return Err("Position out of range");
                }
            }
            return Ok(&(*curr).val);
        }
    }

    pub fn get_length(&self) -> usize {
        //we don't need a semicolon on every line, Rust can interpret this as a usize
        self.len
    }

    pub fn insert(&mut self, position: usize, elem: T) -> Result<(), &str> {
        todo!();
    }

    pub fn is_empty(&self) -> bool {
        //we don't need a semicolon on every line, Rust can interpret this as a bool
        self.len == 0
    }

    pub fn remove(&mut self, position: usize) -> Result<(), &str> {
        todo!();
    }

    pub fn replace(&mut self, position: usize, elem: T) -> Result<(), &str> {
        if position >= self.len {
            return Err("Position out of range");
        }
        unsafe {
            let mut curr = self.head;
            for _i in 0..position {
                if curr != null_mut() {
                    curr = (*curr).next;
                }
                else {
                    return Err("Position out of range");
                }
            }
            (*curr).val = elem;
            Ok(())
        }
    }
}